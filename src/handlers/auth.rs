use std::sync::Arc;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use oauth2::{AuthorizationCode, CsrfToken};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{database::profile::Profile, extensions::google_auth::GoogleAuth};

use super::AuthRequest;

pub async fn auth_handler(Extension(google_auth): Extension<Arc<GoogleAuth>>) -> impl IntoResponse {
    let auth_url = google_auth.auth_url().await;
    Redirect::to(auth_url.as_str())
}

pub async fn auth_callback_handler(
    Extension(google_auth): Extension<Arc<GoogleAuth>>,
    Query(query): Query<AuthRequest>,
    Extension(db_pool): Extension<Pool<Postgres>>,
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
        let profile = Profile::new(
            userinfo.sub,
            userinfo.name.unwrap_or_default(),
            userinfo.email.unwrap_or_default(),
            userinfo.picture,
            token_info.refresh_token,
        );
        let res = profile.save(db_pool).await.unwrap();
        return Json(json!({ "google": res }));
    }
    Json(json!({"error":"failed to get tokens"}))
}
