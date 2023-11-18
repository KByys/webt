use axum::Json;
use serde::{ser::SerializeStruct, Serialize};


pub struct ResponseData<T> {
    /// extra status information about the response
    status: i32,
    data: T,
}

impl<T: serde::Serialize> axum::response::IntoResponse for ResponseData<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> Serialize for ResponseData<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Response", 2)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}
impl<T: Serialize> ResponseData<T> {
    pub fn new(status: i32, data: T) -> ResponseData<T> {
        Self { status, data }
    }
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn data(&self) -> &T {
        &self.data
    }
}
