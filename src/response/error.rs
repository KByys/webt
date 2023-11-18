use std::fmt::Debug;

use axum::http::status::StatusCode;
use axum::Json;
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

impl<T: serde::Serialize> axum::response::IntoResponse for ResponseError<T> {
    fn into_response(self) -> axum::response::Response {
        let mut response = Json(self.body).into_response();
        *response.status_mut() = self.status;
        response
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
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body,
        }
    }
    pub fn new(status: StatusCode, body: T) -> Self {
        Self { status, body }
    }
}
