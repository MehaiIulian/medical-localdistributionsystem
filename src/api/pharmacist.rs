use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::incident::{MedicalNameRecord, PharmacistRecommendation};
use crate::services::gds_router::GdsRouter;

#[derive(Debug, Deserialize)]
pub struct ClaimPayload {
    pub pharmacist_name: String,
    pub pharmacy_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecommendationPayload {
    pub pharmacist_name: String,
    pub pharmacy_name: String,
    pub action_plan_ro: String,
    pub home_cabinet_advice: Option<String>,
    pub recommended_otc_products: Vec<String>,
    pub precautions: Vec<String>,
    pub referral_provider_id: Option<Uuid>,
}

pub async fn list_queue(State(router): State<GdsRouter>) -> Json<Vec<MedicalNameRecord>> {
    let pending = router.list_pending_incidents();
    Json(pending)
}

pub async fn claim_incident(
    State(router): State<GdsRouter>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ClaimPayload>,
) -> Result<Json<MedicalNameRecord>, (StatusCode, Json<Value>)> {
    match router.claim_incident(&id, &payload.pharmacist_name, &payload.pharmacy_name) {
        Ok(mnr) => Ok(Json(mnr)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )),
    }
}

pub async fn submit_recommendation(
    State(router): State<GdsRouter>,
    Path(id): Path<Uuid>,
    Json(payload): Json<RecommendationPayload>,
) -> Result<Json<MedicalNameRecord>, (StatusCode, Json<Value>)> {
    let rec = PharmacistRecommendation {
        pharmacist_id: Uuid::new_v4(),
        pharmacist_name: payload.pharmacist_name,
        pharmacy_name: payload.pharmacy_name,
        action_plan_ro: payload.action_plan_ro,
        home_cabinet_advice: payload.home_cabinet_advice,
        recommended_otc_products: payload.recommended_otc_products,
        precautions: payload.precautions,
        referral_provider_id: payload.referral_provider_id,
        completed_at: Utc::now(),
    };

    match router.resolve_incident(&id, rec) {
        Ok(mnr) => Ok(Json(mnr)),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )),
    }
}

