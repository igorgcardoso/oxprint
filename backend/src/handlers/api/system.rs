use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use crate::{AppState, utils::AppResult};

/// Health check endpoint
pub async fn health(State(state): State<AppState>) -> AppResult<Json<Value>> {
    state.database.health_check().await?;

    Ok(Json(json!({
        "status": "health",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// System status endpoint
pub async fn status(State(state): State<AppState>) -> AppResult<Json<Value>> {
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": uptime,
        "database": "connected",
        "static_files": state.settings.server.static_dir.exists()
    })))
}
