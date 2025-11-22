use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use crate::error::{Result, Error};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(login_api))
}

async fn login_api(payload: Json<PayloadLogin>) -> Result<Json<Value>> {
    if payload.name != "Danton" || payload.pwd != "123" {
        return Err(Error::LoginFail)
    }

    let body = Json(json!(
        {
            "result": {
                "success": true
            }
        }
    ));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct PayloadLogin {
    name: String,
    pwd: String,
}