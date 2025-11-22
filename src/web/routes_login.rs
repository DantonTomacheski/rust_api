use axum::{Json, Router, routing::post};
use serde::Deserialize;
use crate::error::{Error, Result};
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {

    if payload.name != "Danton" || payload.pwd != "123" {
        return Err(Error::LoginFail)
    }

    let body = Json(json!(
        {
            "result": {
                "sucess": true
            }
        }
    ));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    name: String,
    pwd: String,
}
