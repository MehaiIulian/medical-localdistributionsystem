use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatientCategory {
    AdultSelf,
    Child,
    Elderly,
    PetDog,
    PetCat,
    PetOther,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UrgencyLevel {
    GreenMild,     // OTC guidance / pharmacy visit
    YellowModerate,// Pharmacist consultation recommended
    OrangeUrgent,  // Immediate clinic visit needed
    RedEmergency,  // Direct 112 / Emergency Room warning
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientIntakePayload {
    pub patient_category: PatientCategory,
    pub patient_age: Option<u8>,
    pub main_complaint: String,
    pub symptom_tags: Vec<String>,
    pub duration_hours: u32,
    pub pain_scale_1_to_10: u8,
    pub known_allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
    pub current_medications_at_home: Vec<String>,
    pub user_location_lat: Option<f64>,
    pub user_location_lng: Option<f64>,
    pub city: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedFlagAlert {
    pub is_critical: bool,
    pub warning_message_ro: String,
    pub recommended_action: String, // e.g. "Sunați imediat la 112"
}

