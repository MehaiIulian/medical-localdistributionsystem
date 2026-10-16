use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use parking_lot::RwLock;
use uuid::Uuid;

use crate::models::gds::{BusyLevel, NetworkChain, Provider, ProviderType, ResourceAgent};
use crate::models::incident::{IncidentStatus, MedicalNameRecord, PharmacistRecommendation};
use crate::models::triage::{PatientIntakePayload, UrgencyLevel};
use super::ai_triage::AiTriageEngine;

#[derive(Clone)]
pub struct GdsRouter {
    providers: Arc<RwLock<HashMap<Uuid, Provider>>>,
    resources: Arc<RwLock<HashMap<Uuid, ResourceAgent>>>,
    incidents: Arc<RwLock<HashMap<Uuid, MedicalNameRecord>>>,
}

impl GdsRouter {
    pub fn new() -> Self {
        let router = Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            resources: Arc::new(RwLock::new(HashMap::new())),
            incidents: Arc::new(RwLock::new(HashMap::new())),
        };

        router.seed_initial_providers();
        router
    }

    fn seed_initial_providers(&self) {
        let mut p_map = self.providers.write();
        let mut r_map = self.resources.write();

        let initial_providers = vec![
            Provider {
                id: Uuid::new_v4(),
                name: "Farmacia Tei - Barbu Văcărescu".to_string(),
                provider_type: ProviderType::Pharmacy,
                network: NetworkChain::FarmaciaTei,
                county: "București".to_string(),
                city: "Sector 2".to_string(),
                address: "Bulevardul Barbu Văcărescu 120".to_string(),
                latitude: 44.4623,
                longitude: 26.1044,
                phone: "+40 21 230 40 50".to_string(),
                is_non_stop: true,
                is_active_on_call: true,
                current_queue_load: 1,
                busy_level: BusyLevel::Low,
                estimated_wait_minutes: 4,
                google_maps_url: Some("https://www.google.com/maps/search/?api=1&query=Farmacia+Tei+Barbu+Vacarescu+120".to_string()),
                verified: true,
            },
            Provider {
                id: Uuid::new_v4(),
                name: "Catena Non-Stop Piața Romană".to_string(),
                provider_type: ProviderType::Pharmacy,
                network: NetworkChain::Catena,
                county: "București".to_string(),
                city: "Sector 1".to_string(),
                address: "Piața Romană 9".to_string(),
                latitude: 44.4448,
                longitude: 26.0975,
                phone: "+40 800 800 020".to_string(),
                is_non_stop: true,
                is_active_on_call: true,
                current_queue_load: 0,
                busy_level: BusyLevel::Low,
                estimated_wait_minutes: 2,
                google_maps_url: Some("https://www.google.com/maps/search/?api=1&query=Catena+Piata+Romana+9".to_string()),
                verified: true,
            },
            Provider {
                id: Uuid::new_v4(),
                name: "Dr. Max Farmacie Cluj Central".to_string(),
                provider_type: ProviderType::Pharmacy,
                network: NetworkChain::DrMax,
                county: "Cluj".to_string(),
                city: "Cluj-Napoca".to_string(),
                address: "Piața Unirii 4".to_string(),
                latitude: 46.7702,
                longitude: 23.5898,
                phone: "+40 364 400 500".to_string(),
                is_non_stop: true,
                is_active_on_call: true,
                current_queue_load: 2,
                busy_level: BusyLevel::Moderate,
                estimated_wait_minutes: 12,
                google_maps_url: Some("https://www.google.com/maps/search/?api=1&query=Dr+Max+Piata+Unirii+4+Cluj".to_string()),
                verified: true,
            },
            Provider {
                id: Uuid::new_v4(),
                name: "Spitalul Veterinar Non-Stop București (Urgențe Vet)".to_string(),
                provider_type: ProviderType::VetClinic,
                network: NetworkChain::Independent,
                county: "București".to_string(),
                city: "Sector 3".to_string(),
                address: "Șoseaua Mihai Bravu 245".to_string(),
                latitude: 44.4265,
                longitude: 26.1325,
                phone: "+40 722 000 999".to_string(),
                is_non_stop: true,
                is_active_on_call: true,
                current_queue_load: 1,
                busy_level: BusyLevel::Low,
                estimated_wait_minutes: 5,
                google_maps_url: Some("https://www.google.com/maps/search/?api=1&query=Spitalul+Veterinar+Non-Stop+Bucuresti".to_string()),
                verified: true,
            },
        ];

        for p in initial_providers {
            let res_id = Uuid::new_v4();
            let role_name = if p.provider_type == ProviderType::VetClinic {
                "Dr. Medic Veterinar Titular"
            } else {
                "Farmacist Clinician Diriginte"
            };

            r_map.insert(
                res_id,
                ResourceAgent {
                    id: res_id,
                    provider_id: p.id,
                    name: format!("Personal GDS Gardă ({})", p.name),
                    role: role_name.to_string(),
                    license_code: format!("RO-COL-{}", &res_id.to_string()[..6]),
                    is_online: true,
                    current_incident_id: None,
                },
            );

            p_map.insert(p.id, p);
        }
    }

    pub fn list_providers(&self) -> Vec<Provider> {
        let map = self.providers.read();
        map.values().cloned().collect()
    }

    pub fn register_provider(&self, provider: Provider) {
        let mut map = self.providers.write();
        map.insert(provider.id, provider);
    }

    pub fn create_incident(&self, intake: PatientIntakePayload) -> MedicalNameRecord {
        let (urgency, red_flags, sbar_brief) = AiTriageEngine::evaluate(&intake);

        let incident_id = Uuid::new_v4();
        let access_token = format!("MNR-{}", &incident_id.to_string()[..8].to_uppercase());

        let matched = self.find_matching_providers(&intake.city, &urgency);

        let initial_status = if urgency == UrgencyLevel::RedEmergency {
            IncidentStatus::EscalatedToEmergency112
        } else {
            IncidentStatus::AiTriagedWaitingPharmacist
        };

        let mnr = MedicalNameRecord {
            id: incident_id,
            access_token,
            created_at: Utc::now(),
            intake,
            urgency,
            red_flags,
            sbar_brief,
            status: initial_status,
            assigned_pharmacist_id: None,
            assigned_pharmacist_name: None,
            assigned_pharmacy_name: None,
            nearby_matched_pharmacies: matched,
            recommendation: None,
            video_room_id: Some(format!("room-{incident_id}")),
        };

        let mut inc_map = self.incidents.write();
        inc_map.insert(incident_id, mnr.clone());
        mnr
    }

    pub fn get_incident(&self, id: &Uuid) -> Option<MedicalNameRecord> {
        let map = self.incidents.read();
        map.get(id).cloned()
    }

    pub fn get_incident_by_token(&self, token: &str) -> Option<MedicalNameRecord> {
        let map = self.incidents.read();
        map.values().find(|inc| inc.access_token == token).cloned()
    }

    pub fn list_pending_incidents(&self) -> Vec<MedicalNameRecord> {
        let map = self.incidents.read();
        map.values()
            .filter(|inc| {
                matches!(
                    inc.status,
                    IncidentStatus::AiTriagedWaitingPharmacist | IncidentStatus::IntakeSubmitted
                )
            })
            .cloned()
            .collect()
    }

    pub fn claim_incident(
        &self,
        incident_id: &Uuid,
        pharmacist_name: &str,
        pharmacy_name: &str,
    ) -> Result<MedicalNameRecord, &'static str> {
        let mut map = self.incidents.write();
        let inc = map.get_mut(incident_id).ok_or("Incident not found")?;

        inc.assigned_pharmacist_name = Some(pharmacist_name.to_string());
        inc.assigned_pharmacy_name = Some(pharmacy_name.to_string());
        inc.status = IncidentStatus::LiveSessionActive;

        Ok(inc.clone())
    }

    pub fn resolve_incident(
        &self,
        incident_id: &Uuid,
        rec: PharmacistRecommendation,
    ) -> Result<MedicalNameRecord, &'static str> {
        let mut map = self.incidents.write();
        let inc = map.get_mut(incident_id).ok_or("Incident not found")?;

        inc.recommendation = Some(rec);
        inc.status = IncidentStatus::ResolvedWithRecommendation;

        Ok(inc.clone())
    }

    fn find_matching_providers(&self, _city: &str, _urgency: &UrgencyLevel) -> Vec<Provider> {
        let map = self.providers.read();
        let mut list: Vec<Provider> = map.values().filter(|p| p.is_active_on_call).cloned().collect();
        list.sort_by_key(|p| p.current_queue_load);
        list
    }
}

