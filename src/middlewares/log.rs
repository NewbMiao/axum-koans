use axum::body::Bytes;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use hyper::Body;
use tracing::info;

pub async fn log(
    req: Request<axum::body::Body>,
    next: Next<axum::body::Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut no_log = false;

    let path = &req.uri().path().to_string();

    // Don't log these extensions
    let extension_skip = vec![".js", ".html", ".css", ".png", ".jpeg"];
    for ext in extension_skip {
        if path.ends_with(ext) {
            no_log = true;
            break;
        }
    }

    let (req_parts, req_body) = req.into_parts();

    // Print request
    let bytes = buffer_and_print("request", path, req_body, no_log).await?;
    let req = Request::from_parts(req_parts, hyper::Body::from(bytes));

    let res = next.run(req).await;

    let (mut res_parts, res_body) = res.into_parts();

    // Print response
    let bytes = buffer_and_print("response", path, res_body, no_log).await?;

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
                info!(
                    "{} for req: {} with body: {}...",
                    direction,
                    path,
                    &body[0..2000]
                );
            } else {
                info!("{} for req: {} with body: {}", direction, path, body);
            }
        }
    }

    Ok(bytes)
}
