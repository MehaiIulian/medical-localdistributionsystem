use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::gds::Provider;
use super::triage::{PatientIntakePayload, RedFlagAlert, UrgencyLevel};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    IntakeSubmitted,
    AiTriagedWaitingPharmacist,
    PharmacistClaimed,
    LiveSessionActive,
    ResolvedWithRecommendation,
    EscalatedToEmergency112,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbarBrief {
    pub situation: String,     // S: What is happening right now
    pub background: String,    // B: Patient age, duration, allergies, chronic context
    pub assessment: String,    // A: AI analysis, cabinet drugs safety, potential interactions
    pub recommendation: String,// R: Suggested pharmacist verification focus
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacistRecommendation {
    pub pharmacist_id: Uuid,
    pub pharmacist_name: String,
    pub pharmacy_name: String,
    pub action_plan_ro: String,
    pub home_cabinet_advice: Option<String>,
    pub recommended_otc_products: Vec<String>,
    pub precautions: Vec<String>,
    pub referral_provider_id: Option<Uuid>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalNameRecord {
    pub id: Uuid,                  // MNR reference code (e.g. MNR-8F29A)
    pub access_token: String,      // Session token for patient client
    pub created_at: DateTime<Utc>,
    pub intake: PatientIntakePayload,
    pub urgency: UrgencyLevel,
    pub red_flags: Option<RedFlagAlert>,
    pub sbar_brief: SbarBrief,
    pub status: IncidentStatus,
    pub assigned_pharmacist_id: Option<Uuid>,
    pub assigned_pharmacist_name: Option<String>,
    pub assigned_pharmacy_name: Option<String>,
    pub nearby_matched_pharmacies: Vec<Provider>,
    pub recommendation: Option<PharmacistRecommendation>,
    pub video_room_id: Option<String>,
}

