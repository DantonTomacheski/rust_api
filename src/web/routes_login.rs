use crate::{
    error::{Error, Result},
    web,
};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};
use tracing::info;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<PayloadLogin>) -> Result<Json<Value>> {
    info!("{:<12} - api_login", "HANDLER");

    if payload.name != "Danton" || payload.pwd != "1234" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real user token
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "sucess": true
        }
    }));

    info!("Cookies generated: {:?}", cookies.get(web::AUTH_TOKEN));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct PayloadLogin {
    name: String,
    pwd: String,
}
