use std::sync::Arc;

use axum::{routing::get, Router};
use handlers::ping::get_ping;
use tower::ServiceBuilder;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

use crate::{handlers, state::State};

pub fn get_router(state: Arc<State>) -> Router {
    Router::new()
        .route("/", get(get_ping))
        .layer(AddExtensionLayer::new(state))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
