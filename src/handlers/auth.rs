use std::sync::Arc;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use oauth2::{AuthorizationCode, CsrfToken};
use serde::Deserialize;
use serde_json::json;

use crate::extensions::google_auth::GoogleAuth;

pub async fn auth_handler(Extension(google_auth): Extension<Arc<GoogleAuth>>) -> impl IntoResponse {
    let auth_url = google_auth.auth_url().await;
    Redirect::to(auth_url.as_str())
}
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}
pub async fn auth_callback_handler(
    Extension(google_auth): Extension<Arc<GoogleAuth>>,
    Query(query): Query<AuthRequest>,
) -> impl IntoResponse {
    if let Some(token_info) = google_auth
        .get_tokens(
            AuthorizationCode::new(query.code),
            CsrfToken::new(query.state),
        )
        .await
    {
        let userinfo = google_auth
            .get_user_info(token_info.clone().access_token)
            .await;

        return Json(json!({ "token_info": token_info, "user_info":userinfo}));
    }
    Json(json!({"error":"failed to get tokens"}))
}
