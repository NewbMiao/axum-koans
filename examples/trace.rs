use axum::{routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::{event, instrument, Level};

#[instrument]
async fn handler() -> &'static str {
    event!(Level::INFO, "handler event");
    // let span = info_span!("handler");

    // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
    // let _enter = span.enter();
    event!(Level::INFO, "something happened inside handler");
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        // .json()
        .init();

    let app = Router::new().route("/", get(handler)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );
    tracing::info!("listening on {}", 3000);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
