use std::sync::Arc;

use crate::{errors::ServerError, extensions::keycloak_auth::KeycloakAuth};
use axum::{
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Extension, Json,
};

pub async fn user_handler(
    Extension(keycloak_auth): Extension<Arc<KeycloakAuth>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, ServerError> {
    let res = keycloak_auth
        .introspect_token(bearer.token().to_string())
        .await?;
    Ok(Json(res))
}
