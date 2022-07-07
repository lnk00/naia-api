use crate::state::State;
use axum::{extract, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
    version: String,
    name: String,
    databases: Vec<String>,
}

pub async fn get_ping(
    state: extract::Extension<Arc<State>>,
) -> Result<Json<PingResponse>, StatusCode> {
    let res = state.db.list_database_names(None, None).await;

    match res {
        Ok(db_names) => Ok(Json(PingResponse {
            version: "1.0.0".to_owned(),
            name: "naïa API".to_owned(),
            databases: db_names,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
