use axum::{response::IntoResponse, routing::get, Json, Router};
use hyper::Method;
use serde_json::json;
use tower_http::cors::CorsLayer;
#[tokio::main]
async fn main() {
    let origins = ["https://example.com".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET]);
    let app = Router::new().route("/", get(root)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    Json(json!({"result": "hello world"}))
}
