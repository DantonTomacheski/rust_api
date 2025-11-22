use axum::{Json, Router, routing::{post}};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::info;
use crate::error2::{Result, Error};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}

async fn api_login(payload: Json<PayloadParams>) -> Result<Json<Value>> {
    if payload.name != "Danton" || payload.pwd != "213" {
        return Err(Error::LoginFail)
    }

    info!("payload result: {:?}", &payload);

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct PayloadParams {
    name: String,
    pwd: String,
}