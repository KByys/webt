use crate::content_type::ContentType;
use hyper::header::HeaderValue;
use hyper::header::{ACCESS_CONTROL_EXPOSE_HEADERS, CONTENT_DISPOSITION, CONTENT_TYPE};
use hyper::{Body, Response, StatusCode};
use std::fs::read;
use std::io::Result;
use std::path::{Path, PathBuf};

type BodyResponse = hyper::Response<Body>;
#[derive(Default)]
pub struct FileResponse {
    pub body: BodyResponse,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for FileResponse {
    fn into_response(self) -> axum::response::Response {
        use axum::http::HeaderValue;
        use axum::response::{IntoResponse, Response};
        self.body.into_response()
    }
}

impl FileResponse {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file_name = match path.as_ref().file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            _ => String::new(),
        };
        Self::open_and_set(file_name, path)
    }
    pub fn open_and_set(file_name: impl AsRef<str>, path: impl AsRef<Path>) -> Result<Self> {
        let buf = read(path)?;

        let path = PathBuf::from(file_name.as_ref());
        let content_type = ContentType::from_extension(path.extension()).unwrap_or("text/plain");
        let file_name: String =
            url::form_urlencoded::byte_serialize(file_name.as_ref().as_bytes()).collect();
        let content_disposition = format!("attachment; filename={}", file_name);
        let body = Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static(content_type))
            .header(
                CONTENT_DISPOSITION,
                HeaderValue::from_str(content_disposition.as_str()).unwrap(),
            )
            .header(ACCESS_CONTROL_EXPOSE_HEADERS, CONTENT_DISPOSITION)
            .body(Body::from(buf))
            .unwrap();
        Ok(Self { body })
    }
    pub fn not_found_with(body: Body) -> Self {
        let body = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap();
        Self { body }
    }
}
