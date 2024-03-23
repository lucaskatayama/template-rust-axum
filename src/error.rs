use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Fail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
    }
}
