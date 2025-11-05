use std::net::SocketAddr;

use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug};
#[cfg(debug_assertions)]
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(
        #[cfg(debug_assertions)]
        Level::DEBUG,
        #[cfg(not(debug_assertions))]
        Level::INFO,
    )
    .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health_check))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    debug!("OxPrint backend server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({
        "message": "Welcome to OxPrint API",
        "version": "0.1.0",
        "status": "running"
    }))
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
