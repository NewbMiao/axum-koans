use std::net::SocketAddr;

use axum::body::Bytes;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{middleware, Json, Router};
use hyper::Body;
use serde::Deserialize;
use serde_json::json;
use tracing_subscriber::FmtSubscriber;

pub async fn log_request_response(
    req: Request<axum::body::Body>,
    next: Next<axum::body::Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut do_log = true;

    let path = &req.uri().path().to_string();

    // Don't log these extensions
    let extension_skip = vec![".js", ".html", ".css", ".png", ".jpeg"];
    for ext in extension_skip {
        if path.ends_with(ext) {
            do_log = false;
            break;
        }
    }

    // Want to skip logging these paths
    let skip_paths = vec!["/example/path"];
    for skip_path in skip_paths {
        if path.ends_with(skip_path) {
            do_log = false;
            break;
        }
    }

    let (req_parts, req_body) = req.into_parts();

    // Print request
    let bytes = buffer_and_print("request", path, req_body, do_log).await?;
    let req = Request::from_parts(req_parts, hyper::Body::from(bytes));

    let res = next.run(req).await;

    let (mut res_parts, res_body) = res.into_parts();

    // Print response
    let bytes = buffer_and_print("response", path, res_body, do_log).await?;

    // When your encoding is chunked there can be problems without removing the header
    res_parts.headers.remove("transfer-encoding");

    let res = Response::from_parts(res_parts, Body::from(bytes));

    Ok(res)
}

// Consumes body and prints
async fn buffer_and_print<B>(
    direction: &str,
    path: &str,
    body: B,
    log: bool,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        if log && !body.is_empty() {
            if body.len() > 2000 {
                println!(
                    "{} for req: {} with body: {}...",
                    direction,
                    path,
                    &body[0..2000]
                );
            } else {
                println!("{} for req: {} with body: {}", direction, path, body);
            }
        }
    }

    Ok(bytes)
}

#[tokio::test]
async fn test_log_request_response() {
    // create a request to be passed to the middleware
    let req = Request::new(Body::from("Hello, Axum!"));

    // create a simple router to test the middleware
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware::from_fn(log_request_response));

    // send the request through the middleware
    let res = app.clone().oneshot(req).await.unwrap();

    // make sure the response has a status code of 200
    assert_eq!(res.status(), StatusCode::OK);
}
#[derive(Debug, Deserialize)]
struct UserRequest {
    user_id: u32,
}
async fn hello_world(Json(user_req): Json<UserRequest>) -> impl IntoResponse {
    Json(json!({"res":"Hello, World!", "user_id": user_req.user_id}))
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route("/", post(hello_world))
        .layer(middleware::from_fn(log_request_response));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
