use axum::{middleware, routing::get, Extension, Router};
use axum_koans::{
    database::get_db_pool,
    extensions::{google_auth::GoogleAuth, keycloak_auth::KeycloakAuth},
    handlers::{
        auth::{auth_callback_handler, auth_handler},
        login::{login_callback_handler, login_handler},
    },
    middlewares::log,
};
use dotenvy::{self, dotenv};
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

fn load_env() {
    if let Err(e) = dotenv() {
        eprintln!("Failed to load .env file: {}", e);
    }
}
#[tokio::main]
async fn main() {
    load_env();
    let google_client_id =
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET")
        .expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");

    let keycloak_client_id = env::var("KEYCLOAK_CLIENT_ID")
        .expect("Missing the KEYCLOAK_CLIENT_ID environment variable.");
    let keycloak_client_secret = env::var("KEYCLOAK_CLIENT_SECRET")
        .expect("Missing the KEYCLOAK_CLIENT_SECRET environment variable.");

    let database_url =
        env::var("DATABASE_URL").expect("Missing the DATABASE_URL environment variable.");

    // db pool
    let db_pool = get_db_pool(database_url).await.unwrap();
    // Start configuring a `fmt` subscriber
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
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
                        .layer(Extension(Arc::new(KeycloakAuth::new(
                            &keycloak_client_id,
                            &keycloak_client_secret,
                        ))))
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
                .layer(Extension(Arc::new(GoogleAuth::new(
                    &google_client_id,
                    &google_client_secret,
                ))))
                .layer(Extension(db_pool))
                .into_inner(),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(middleware::from_fn(log));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
