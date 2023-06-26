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

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::{routing::get, Extension, Router};
    use hyper::{http::HeaderValue, Request, StatusCode};
    use tower::{ServiceBuilder, ServiceExt};

    use crate::{
        config::Config, errors::ServerError, extensions::keycloak_auth::KeycloakAuth,
        handlers::user::user_handler,
    };

    #[tokio::test]
    async fn test_log_request_response() -> Result<(), ServerError> {
        let req = Request::builder()
            .method("GET")
            .uri("/")
            .header(
                "Authorization",
                HeaderValue::from_static("Bearer test.test.test"),
            )
            .body(hyper::Body::empty())
            .unwrap();

        let config = Config::from_env()?;
        // create a simple router to test the middleware
        let app = Router::new().route("/", get(user_handler)).layer(
            ServiceBuilder::new()
                .layer(Extension(Arc::new(KeycloakAuth::new(config.keycloak))))
                .into_inner(),
        );

        // send the request through the middleware
        let res = app.clone().oneshot(req).await.unwrap();

        // make sure the response has a status code of 200
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }
}
