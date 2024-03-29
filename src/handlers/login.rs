use std::sync::Arc;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use oauth2::{AuthorizationCode, CsrfToken};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::warn;

use crate::{
    errors::ServerError,
    extensions::{google_auth::GoogleAuth, keycloak_auth::KeycloakAuth, KeyCloakIdp},
};

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
) -> Result<impl IntoResponse, ServerError> {
    let token_info = keycloak_auth
        .get_tokens(
            AuthorizationCode::new(query.code),
            CsrfToken::new(query.state),
        )
        .await?;
    warn!("keycloak_token: {:?}", token_info);

    let userinfo = keycloak_auth
        .get_user_info(token_info.clone().access_token)
        .await?;

    let broker_token = keycloak_auth
        .get_idp_token(
            token_info.clone().access_token,
            KeyCloakIdp::Google.as_str(),
        )
        .await?;
    warn!("broker_token: {:?}", broker_token);

    let google_tokens = keycloak_auth
        .token_exchange(
            token_info.clone().access_token,
            KeyCloakIdp::Google.as_str(),
        )
        .await?;
    warn!("google_tokens: {:?}", google_tokens);

    let google_info = google_auth
        .get_user_info(google_tokens.access_token)
        .await?;
    Ok(Json(json!({ "google": google_info, "keycloak":userinfo })))
}
