use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router, Server};
use hyper::{Body, Request, StatusCode};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn logging_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, String)> {

    println!("Method: {:?}", req.method());
    println!("Headers: {:?}", req.headers());
    println!("Body: {:?}", req.uri());

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let body_str = hyper::body::to_bytes(body).await.unwrap().to_vec();
    println!(
        "Sending response body: {}",
        String::from_utf8_lossy(&body_str)
    );

    Ok(Response::from_parts(parts, Body::from(body_str)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .layer(from_fn(logging_middleware));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
