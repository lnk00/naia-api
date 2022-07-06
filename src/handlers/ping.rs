use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
    version: String,
    name: String,
}

pub async fn get_ping() -> Json<PingResponse> {
    let res: PingResponse = PingResponse {
        version: "1.0.0".to_owned(),
        name: "naïa API".to_owned(),
    };
    Json(res)
}
