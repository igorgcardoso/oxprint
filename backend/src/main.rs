use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::{trace::TraceLayer};
use tracing::{debug};
#[cfg(debug_assertions)]
use tracing::Level;

use crate::{config::Settings, database::Database};

mod config;
mod handlers;
mod services;
mod models;
mod database;
mod middleware;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(
        #[cfg(debug_assertions)]
        Level::DEBUG,
        #[cfg(not(debug_assertions))]
        Level::INFO,
    )
    .init();

    let settings = Settings::load().map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;

    let database = Database::new(&settings.database.url).await?;

    let app = create_app(database, settings.clone()).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));

    debug!("OxPrint backend server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}

async fn create_app(database: Database, settings: Settings) -> Router {
    use handlers::{api, static_files, websocket};
    use middleware::cors;
    use tower_http::compression::CompressionLayer;

    Router::new()
        // Websocket endpoint
        .route("/ws", get(websocket::handler))

        // API routes
        .nest("/api", api::routes())

        // Static files serving with compression
        .nest_service("/assets", static_files::assets_service())

        // SPA fallback - must be last
        .fallback(static_files::spa_handler)

        // Middleware layers
        .layer(CompressionLayer::new())
        .layer(cors::layer())
        .layer(TraceLayer::new_for_http())

        // Share state across handlers
        .with_state(AppState {
            database,
            settings,
        })
}

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Settings,
}
