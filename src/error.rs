use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "not allowed"),
        };

        (status, body).into_response()
    }
}

pub enum Error {
    LoginFail,
}