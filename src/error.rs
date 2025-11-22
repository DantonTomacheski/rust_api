use axum::{http::StatusCode, response::{IntoResponse, Response}};
use tracing::error; 

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!(error = ?self, "Request failed");

        let (status, body) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "You don't have permission")
        };

        (status, body).into_response()
    }
}