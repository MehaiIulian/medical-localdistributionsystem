use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};

use crate::models::gds::Provider;
use crate::services::gds_router::GdsRouter;

pub async fn list_providers(State(router): State<GdsRouter>) -> Json<Vec<Provider>> {
    let providers = router.list_providers();
    Json(providers)
}

pub async fn register_provider(
    State(router): State<GdsRouter>,
    Json(payload): Json<Provider>,
) -> (StatusCode, Json<Provider>) {
    router.register_provider(payload.clone());
    (StatusCode::CREATED, Json(payload))
}

pub async fn gds_network_stats(State(router): State<GdsRouter>) -> Json<Value> {
    let providers = router.list_providers();
    let on_call = providers.iter().filter(|p| p.is_active_on_call).count();
    let pending = router.list_pending_incidents().len();

    Json(json!({
        "status": "operational",
        "gds_protocol_version": "1.0-MEDGDS",
        "active_on_call_providers": on_call,
        "total_registered_providers": providers.len(),
        "pending_triaged_incidents": pending,
        "supported_jurisdictions": ["RO", "EU"]
    }))
}

