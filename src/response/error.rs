use std::fmt::Display;

use hyper::{Body, Response, StatusCode};

#[derive(Debug)]
pub struct ResponseError {
    code: StatusCode,
    message: String,
}
#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        self.to_response().unwrap().into_response()
    }
}
impl<T: Display> From<(StatusCode, T)> for ResponseError {
    fn from(value: (StatusCode, T)) -> Self {
        Self {
            code: value.0,
            message: value.1.to_string(),
        }
    }
}
impl From<std::io::Error> for ResponseError {
    fn from(value: std::io::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.to_string(),
        }
    }
}
impl ResponseError {
    pub fn status_code(&self) -> StatusCode {
        self.code
    }
    pub fn to_response(self) -> Result<Response<Body>, hyper::http::Error> {
        Response::builder()
            .status(self.code)
            .body(Body::from(self.message))
    }
    pub fn INTERNAL_SERVER_ERROR(e: impl Display) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}
