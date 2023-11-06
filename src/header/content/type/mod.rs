mod r#match;
use std::ffi::OsStr;
use std::path::PathBuf;

use hyper::{HeaderMap, header::HeaderValue};

use crate::header::HeaderParserError;
#[derive(Default, Debug, Clone)]
pub struct ContentType {
    inner: String,
}
// #[cfg(features = "axum")]
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for ContentType {
    type Rejection = String;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Self::try_from(&parts.headers).map_err(|err| err.to_string())
    }
}
use hyper::header::CONTENT_TYPE;
impl TryFrom<&HeaderMap> for ContentType {
    type Error = HeaderParserError;

    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        match value.get(CONTENT_TYPE) {
            Some(value) => Ok(Self::from(value.to_str()?)),
            _ => Err(HeaderParserError::MissingHeaderValue(CONTENT_TYPE)),
        }
    }
}

impl From<&str> for ContentType {
    fn from(value: &str) -> Self {
        Self {
            inner: value.into(),
        }
    }
}
use hyper::http::Error;
impl TryFrom<ContentType> for HeaderValue {
    type Error = Error;

    fn try_from(value: ContentType) -> Result<Self, Self::Error> {
        match r#match::match_self(&value.inner) {
            Some(value) => Ok(HeaderValue::from_static(value)),
            _ => Ok(HeaderValue::from_str(&value.inner)?)
        }
    }
}

impl ContentType {
    pub fn from_filename(file_name: impl AsRef<OsStr>) -> Option<Self> {
        let path = PathBuf::from(file_name.as_ref());
        Self::from_extension(path.extension())
    }
    pub fn from_extension(ext: Option<&OsStr>) -> Option<Self> {
        let inner = r#match::from_extension(ext)?;
        Some(Self {
            inner: inner.into(),
        })
    }
    pub fn as_extension(&self) -> Option<&'static str> {
        r#match::as_extension(Some(self.inner.as_str()))
    }
    pub fn content_type(&self) -> &str {
        &self.inner
    }
    pub fn static_content_type(&self, or: &'static str) -> &'static str {
        r#match::match_self(&self.inner).unwrap_or(or)
    }
    
}
