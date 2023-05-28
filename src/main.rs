use axum::{middleware, routing::get, Extension, Router};
use axum_demo::{
    extensions::google_auth::GoogleAuth,
    handlers::auth::{auth_callback_handler, auth_handler},
    middlewares::log,
};
use dotenvy::{self, dotenv};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tracing_subscriber::FmtSubscriber;

fn load_env() {
    if let Err(e) = dotenv() {
        eprintln!("Failed to load .env file: {}", e);
    }
}
#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    load_env();
    let google_client_id =
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET")
        .expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");

    let database_url =
        env::var("DATABASE_URL").expect("Missing the DATABASE_URL environment variable.");

    // db pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let app = Router::new()
        .route("/auth", get(auth_handler))
        .route("/auth-callback", get(auth_callback_handler))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(log))
                .layer(Extension(Arc::new(GoogleAuth::new(
                    &google_client_id,
                    &google_client_secret,
                ))))
                .layer(Extension(db_pool))
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
