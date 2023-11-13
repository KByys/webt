use std::fmt::Debug;

use hyper::{header::CONTENT_TYPE, Body, Response, StatusCode};
use serde_json::json;


pub struct ResponseError<T> {
    status: StatusCode,
    body: T,
}

impl<E: Debug> Debug for ResponseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseError")
            .field("status", &self.status)
            .field("body", &self.body)
            .finish()
    }
}
#[cfg(feature = "axum")]
impl<T: serde::Serialize> axum::response::IntoResponse for ResponseError<T> {
    fn into_response(self) -> axum::response::Response {
        self.to_response().unwrap().into_response()
    }
}



pub trait FromError<E> {
    fn from_error(error: E) -> (StatusCode, Self);
}


impl<E, T> From<E> for ResponseError<T>
where
    E: std::error::Error,
    T: serde::Serialize + FromError<E>,
{
    fn from(value: E) -> Self {
        let (status, body) = T::from_error(value);
        Self { status, body }
    }
}

impl<T: serde::Serialize> ResponseError<T> {
    pub fn status(&self) -> StatusCode {
        self.status
    }
    pub fn body(&self) -> &T {
        &self.body
    }
    pub fn ok(body: T) -> Self {
        Self {
            status: StatusCode::OK,
            body,
        }
    }
    pub fn internal_server_error(body: T) -> Self {
        Self { status: StatusCode::INTERNAL_SERVER_ERROR, body }
    }
    pub fn new(status: StatusCode, body: T) -> Self {
        Self { status, body }
    }
    pub fn to_response(&self) -> Result<Response<Body>, hyper::http::Error> {
        Response::builder()
            .status(self.status)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(json!(self.body).to_string()))
    }
}
