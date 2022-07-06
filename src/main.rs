mod handlers;
mod utils;

use axum::{routing::get, Router};
use handlers::ping::get_ping;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utils::logging::setup_rust_logging;

#[tokio::main]
async fn main() {
    setup_rust_logging();

    let app = Router::new()
        .route("/", get(get_ping))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
