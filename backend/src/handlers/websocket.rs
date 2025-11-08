use axum::{
    debug_handler, extract::{State, WebSocketUpgrade}, response::Response
};
use tracing::info;

#[cfg(debug_assertions)]
use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn handler(
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| async {
        info!("WebSocket connection established");
    })
}
