use axum::{middleware, routing::get, Extension, Router};
use axum_demo::{
    extensions::google_auth::GoogleAuth,
    handlers::auth::{auth_callback_handler, auth_handler},
    middlewares::log,
};
use dotenvy::{self, dotenv};
use std::{env, net::SocketAddr, sync::Arc};
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
    let app = Router::new()
        .route("/auth", get(auth_handler))
        .route(
            "/auth-callback",
            get(auth_callback_handler).layer(middleware::from_fn(log)),
        )
        .layer(Extension(Arc::new(GoogleAuth::new(
            &google_client_id,
            &google_client_secret,
        ))));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
