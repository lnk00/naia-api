mod database;
mod handlers;
mod router;
mod state;
mod utils;

#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use crate::{database::get_database_client, router::get_router};
use dotenv::dotenv;
use state::State;
use std::{error::Error, sync::Arc};
use utils::logging::setup_rust_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    setup_rust_logging();

    let db = get_database_client().await?;
    let state = Arc::new(State { db });
    let app = get_router(state);

    tracing::info!("Server listening on port {}...", 3000);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
