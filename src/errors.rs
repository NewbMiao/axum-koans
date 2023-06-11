use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use sqlx;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Postgre error: {0}")]
    PGError(#[from] sqlx::Error),
    #[error("Config error: {0}")]
    ConfigError(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let body = match self {
            ServerError::PGError(_) => self.to_string(),
            ServerError::ConfigError(_) => self.to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
