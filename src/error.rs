use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Deserialize;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!(Error = ?self, "Request failed");
        let (status, body) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "You are not allowed")
        };

        (status, body).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub enum Error {
    LoginFail,
}

