use std::sync::Arc;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use oauth2::{AuthorizationCode, CsrfToken};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::extensions::{google_auth::GoogleAuth, keycloak_auth::KeycloakAuth, KeyCloakIdp};

use super::AuthRequest;

pub async fn login_handler(
    Extension(keycloak_auth): Extension<Arc<KeycloakAuth>>,
) -> impl IntoResponse {
    let authorize_url = keycloak_auth.auth_url(KeyCloakIdp::Google).await;
    Redirect::to(authorize_url.as_str())
}

pub async fn login_callback_handler(
    Extension(google_auth): Extension<Arc<GoogleAuth>>,
    Extension(keycloak_auth): Extension<Arc<KeycloakAuth>>,
    Query(query): Query<AuthRequest>,
    Extension(_db_pool): Extension<Pool<Postgres>>,
) -> impl IntoResponse {
    if let Some(token_info) = keycloak_auth
        .get_tokens(
            AuthorizationCode::new(query.code),
            CsrfToken::new(query.state),
        )
        .await
    {
        let userinfo = keycloak_auth
            .get_user_info(token_info.clone().access_token)
            .await;
        let google_tokens = keycloak_auth
            .token_exchange(
                token_info.clone().access_token,
                KeyCloakIdp::Google.as_str(),
            )
            .await;
        let google_info = google_auth.get_user_info(google_tokens.access_token).await;

        return Json(json!({ "google": google_info, "keycloak":userinfo }));
    }

    Json(json!({"error":"failed to get tokens"}))
}
