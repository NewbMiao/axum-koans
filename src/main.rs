use axum::{middleware, routing::get, Extension, Router};
use axum_koans::{
    config::Config,
    database::get_db_pool,
    errors::ServerError,
    extensions::{google_auth::GoogleAuth, keycloak_auth::KeycloakAuth},
    handlers::{
        auth::{auth_callback_handler, auth_handler},
        login::{login_callback_handler, login_handler},
    },
    middlewares::log,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{self, TraceLayer},
};
use tracing::{error, Level, Span};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let config = Config::from_env()?;
    // db pool
    let db_pool = get_db_pool(config.database).await?;
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        // .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let app = Router::new()
        .nest(
            "/keycloak", // use keyloak oauth server
            Router::new()
                .route("/login", get(login_handler))
                .route("/login-callback", get(login_callback_handler))
                .layer(
                    ServiceBuilder::new()
                        .layer(Extension(Arc::new(KeycloakAuth::new(config.keycloak))))
                        .into_inner(),
                ),
        )
        .nest(
            "/google", // use google oauth server
            Router::new()
                .route("/auth", get(auth_handler))
                .route("/auth-callback", get(auth_callback_handler)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(Arc::new(GoogleAuth::new(config.google))))
                .layer(Extension(db_pool))
                .into_inner(),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_failure(
                    |err: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                        error!(
                            "something went wrong: error={:?} latency={:?} span={:?}",
                            err, latency, span
                        )
                    },
                ),
        )
        .layer(middleware::from_fn(log));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
