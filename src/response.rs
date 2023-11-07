use crate::header::content::ContentDisposition;
use crate::header::ContentType;
use hyper::header::{ACCESS_CONTROL_EXPOSE_HEADERS, CONTENT_DISPOSITION, CONTENT_TYPE};
use hyper::{Body, Response, StatusCode};
use std::fs::read;
use std::io::Result;
use std::path::Path;

type BodyResponse = hyper::Response<Body>;
#[derive(Default)]
pub struct BodyFile {
    pub body: BodyResponse,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for BodyFile {
    fn into_response(self) -> axum::response::Response {
        self.body.into_response()
    }
}

impl BodyFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file_name = match path.as_ref().file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            _ => String::new(),
        };
        Self::open_and_set(file_name, path)
    }
    pub fn open_and_set(file_name: impl AsRef<str>, path: impl AsRef<Path>) -> Result<Self> {
        let buf = read(path)?;

        let content_type =
            ContentType::from_filename(file_name.as_ref()).expect("The extension is not supported");
        let body = Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, content_type)
            .header(CONTENT_DISPOSITION, ContentDisposition::new(file_name))
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
