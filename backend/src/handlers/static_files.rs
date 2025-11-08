use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};
use tower_http::services::ServeDir;
use std::path::PathBuf;
use crate::AppState;

/// Serve the main SPA HTML file
pub async fn spa_handler(State(state): State<AppState>) -> impl IntoResponse {
    serve_file(state.settings.server.static_dir.clone(), "index.html".to_string()).await
}

/// Serve static assets (CSS, JS, Images)
pub fn assets_service() -> ServeDir {
    ServeDir::new("static/assets")
}

/// Generic file serving helper
fn serve_file(static_dir: PathBuf, file_path: String) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> {
    Box::pin(async move {
        let file_path = static_dir.join(&file_path);

        match tokio::fs::read(&file_path).await {
            Ok(contents) => {
                let mime_type = mime_guess::from_path(&file_path)
                    .first_or_octet_stream();

                let mut response = Response::new(contents.into());
                response.headers_mut().insert(
                    header::CONTENT_TYPE, mime_type.as_ref().parse().unwrap());

                if file_path.extension().map_or(false, |ext| {
                    matches!(ext.to_str(), Some("css" | "js" | "png" | "jpg" | "gif" | "svg"))
                }) {
                    response.headers_mut().insert(
                        header::CACHE_CONTROL, "public, max-age=31536000".parse().unwrap()
                    );
                }
                response
            }
            Err(_) => {
                if file_path.file_name().and_then(|name| name.to_str()) != Some("index.html") {
                    return serve_file(static_dir, "index.html".to_string()).await;
                }

                let html = Html(r#"
                    <html>
                        <body>
                            <h1>OxPrint</h1>
                            <p>Frontend not built. Run the build script to generate static assets.</p>
                        </body>
                    </html>
                "#);

                (StatusCode::NOT_FOUND, html).into_response()
            }
        }
    })
}
