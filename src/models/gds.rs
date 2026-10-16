use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Pharmacy,
    VetPharmacy,
    VetClinic,
    Clinic,
    HospitalEmergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkChain {
    Independent,
    Catena,
    DrMax,
    HelpNet,
    FarmaciaTei,
    Dona,
    MedLife,
    ReginaMaria,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BusyLevel {
    Low,
    Moderate,
    High,
    Overloaded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub provider_type: ProviderType,
    pub network: NetworkChain,
    pub county: String,       // e.g. "București", "Cluj", "Iași"
    pub city: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub phone: String,
    pub is_non_stop: bool,
    pub is_active_on_call: bool,
    pub current_queue_load: usize,
    pub busy_level: BusyLevel,
    pub estimated_wait_minutes: u32,
    pub google_maps_url: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAgent {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub name: String,
    pub role: String, // e.g. "Farmacist Diriginte", "Medic Veterinar", "Farmacist Rezident"
    pub license_code: String, // Colegiul Farmaciștilor / CMV
    pub is_online: bool,
    pub current_incident_id: Option<Uuid>,
}

