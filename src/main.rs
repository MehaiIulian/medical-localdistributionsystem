use std::net::SocketAddr;
use axum::{
    extract::WebSocketUpgrade,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod models;
mod services;

use services::gds_router::GdsRouter;
use services::signaling::handle_socket;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "medgds=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_service = GdsRouter::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        // Patient endpoints
        .route("/patient/triage", post(api::patient::submit_triage))
        .route("/patient/mnr/:identifier", get(api::patient::get_incident_status))
        // Pharmacist Cockpit endpoints
        .route("/pharmacist/queue", get(api::pharmacist::list_queue))
        .route("/pharmacist/claim/:id", post(api::pharmacist::claim_incident))
        .route("/pharmacist/recommend/:id", post(api::pharmacist::submit_recommendation))
        // GDS Distribution network endpoints
        .route("/gds/providers", get(api::gds::list_providers).post(api::gds::register_provider))
        .route("/gds/stats", get(api::gds::gds_network_stats));

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .route(
            "/ws/consultation",
            get(|ws: WebSocketUpgrade| async move {
                ws.on_upgrade(|socket| handle_socket(socket))
            }),
        )
        .fallback_service(ServeDir::new("static"))
        .layer(cors)
        .with_state(router_service);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("MedGDS Healthcare Distribution Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Server failed during execution");
}

