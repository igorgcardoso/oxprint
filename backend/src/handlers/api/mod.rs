use axum::{routing::get, Router};
use crate::AppState;

pub mod system;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(system::health))
        .route("/system/status", get(system::status))
}
