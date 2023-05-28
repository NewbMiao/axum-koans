use axum::{response::IntoResponse, routing::get, Json, Router};
use hyper::Method;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
#[tokio::main]
async fn main() {
    let origins = ["https://example.com".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET]);
    let app = Router::new().route("/", get(root)).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    Json(json!({"result": "hello world"}))
}
