use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

pub fn layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any)
}
