use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::incident::MedicalNameRecord;
use crate::models::triage::PatientIntakePayload;
use crate::services::gds_router::GdsRouter;

pub async fn submit_triage(
    State(router): State<GdsRouter>,
    Json(payload): Json<PatientIntakePayload>,
) -> (StatusCode, Json<MedicalNameRecord>) {
    let incident = router.create_incident(payload);
    (StatusCode::CREATED, Json(incident))
}

pub async fn get_incident_status(
    State(router): State<GdsRouter>,
    Path(identifier): Path<String>,
) -> Result<Json<MedicalNameRecord>, (StatusCode, Json<Value>)> {
    // Check if UUID or MNR token
    let record = if let Ok(uuid) = Uuid::parse_str(&identifier) {
        router.get_incident(&uuid)
    } else {
        router.get_incident_by_token(&identifier)
    };

    match record {
        Some(mnr) => Ok(Json(mnr)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Incident (MNR) not found"})),
        )),
    }
}

