use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};
use serde_json::json;
use sqlx;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Postgre error: {0}")]
    PGError(#[from] sqlx::Error),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("Request token error: {0:?}")]
    RequestTokenError(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error("Request error: {0:?}")]
    RequestError(#[from] reqwest::Error),
    #[error("Parse json error: {0}")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("Invalid request token")]
    InvalidBearerToken,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ServerError::InvalidBearerToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = self.to_string();

        (
            status_code,
            Json(json!({
                "error": body,
            })),
        )
            .into_response()
    }
}
