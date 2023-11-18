mod data;
use std::path::Path;

pub use data::ResponseData;
mod error;
use axum::{
    http::{self, HeaderName},
    response::IntoResponse,
};
pub use error::{FromError, ResponseError};
use http::{header::CONTENT_TYPE, HeaderValue, StatusCode};
use reqwest::header::CONTENT_DISPOSITION;
use serde_json::json;

pub type Response<T, E> = std::result::Result<ResponseData<T>, ResponseError<E>>;
pub type HttpResponse<T> = http::Response<T>;

pub struct Body {
    body: Vec<u8>,
    headers: Vec<(HeaderName, HeaderValue)>,
    status: StatusCode,
}
use crate::header::{ContentDisposition, ContentType, HeaderKey};
use std::io::Result;

impl IntoResponse for Body {
    fn into_response(self) -> axum::response::Response {
        let mut response = self.body.into_response();
        *response.status_mut() = self.status;
        for (name, value) in self.headers {
            response.headers_mut().insert(name, value);
        }
        response
    }
}

impl Body {
    pub fn from_text(text: impl AsRef<str>, status: StatusCode) -> Self {
        Self {
            status,
            body: text.as_ref().as_bytes().into(),
            headers: vec![(CONTENT_TYPE, HeaderValue::from_static("text/plain"))],
        }
    }
    pub fn from_json<S: serde::Serialize>(json: S, status: StatusCode) -> Self {
        Self {
            status,
            body: json!(json).to_string().into_bytes(),
            headers: vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))],
        }
    }
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let filename = path.file_name().and_then(|f| f.to_str());
        let bytes = std::fs::read(&path)?;
        Ok(Self::from_bytes(bytes, filename, StatusCode::OK))
    }
    pub fn from_bytes(
        bytes: impl Into<Vec<u8>>,
        filename: Option<impl AsRef<str>>,
        status: StatusCode,
    ) -> Self {
        if let Some(filename) = filename {
            let disposition = ContentDisposition::from_filename(&filename);
            let content_type = ContentType::from_filename(filename.as_ref()).unwrap_or_default();
            Self {
                body: bytes.into(),
                status,
                headers: vec![
                    (CONTENT_TYPE, content_type.header_value()),
                    (CONTENT_DISPOSITION, disposition.header_value()),
                ],
            }
        } else {
            Self {
                body: bytes.into(),
                status,
                headers: vec![(CONTENT_TYPE, HeaderValue::from_static("text/plain"))],
            }
        }
    }
}
