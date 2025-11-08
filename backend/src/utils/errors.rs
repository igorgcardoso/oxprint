use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("Validation error: {0}")]
        Validation(String),

        #[error("Resource not found: {0}")]
        NotFound(String),

        #[error("Internal server error: {0}")]
        Internal(String),

        #[error("Printer communication error: {0}")]
        PrinterComm(String),

        #[error("File operation error: {0}")]
        FileOperation(#[from] std::io::Error),

        #[error("JSON parsing error: {0}")]
        Json(#[from] serde_json::Error),

        #[error("Configuration error: {0}")]
        Config(#[from] config::ConfigError),
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let (status, error_message) = match &self {
                AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
                AppError::Auth(_) => (StatusCode::UNAUTHORIZED, "Authentication failed"),
                AppError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
                AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Resource not found"),
                AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
                AppError::PrinterComm(_) => (StatusCode::SERVICE_UNAVAILABLE, "Printer unavailable"),
                AppError::FileOperation(_) => (StatusCode::INTERNAL_SERVER_ERROR, "File operation failed"),
                AppError::Json(_) => (StatusCode::BAD_REQUEST, "Invalid JSON"),
                AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            };

            let body = Json(json!({
                "error": error_message,
                "details": self.to_string(),
                "status": status.as_u16()
            }));

            (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
