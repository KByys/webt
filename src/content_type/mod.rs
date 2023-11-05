mod r#match;
use std::ffi::OsStr;

use hyper::{header::HeaderValue, HeaderMap};
#[derive(Default, Debug, Clone)]
pub struct ContentType {
    inner: Option<&'static str>,
}
#[cfg(features = "axum")]
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for ContentType {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from(&parts.headers))
    }
}

impl From<&HeaderMap> for ContentType {
    fn from(value: &HeaderMap) -> Self {
        if let Some(value) = value.get("Content-Type") {
            Self::from(value)
        } else {
            Default::default()
        }
    }
}

impl From<Option<&HeaderValue>> for ContentType {
    fn from(value: Option<&HeaderValue>) -> Self {
        if let Some(value) = value {
            Self::from(value)
        } else {
            Default::default()
        }
    }
}

impl From<&HeaderValue> for ContentType {
    fn from(value: &HeaderValue) -> Self {
        let inner = if let Ok(ty) = value.to_str() {
            r#match::match_self(ty)
        } else {
            None
        };
        Self { inner }
    }
}
impl From<&str> for ContentType {
    fn from(value: &str) -> Self {
        let inner = r#match::match_self(value);
        Self { inner }
    }
}

impl From<String> for ContentType {
    fn from(value: String) -> Self {
        let inner = r#match::match_self(&value);
        Self { inner }
    }
}
impl ContentType {
    pub fn from_static(value: &'static str) -> Self {
        Self { inner: Some(value) }
    }
    pub fn from_extension(ext: Option<&OsStr>) -> Self {
        let inner = r#match::from_extension(ext);
        Self { inner }
    }
    pub fn as_extension(&self) -> Option<&'static str> {
        r#match::as_extension(self.inner)
    }
    pub fn content_type(&self) -> Option<&'static str> {
        self.inner
    }
}

pub fn from_extension(ext: Option<&OsStr>) -> Option<&'static str> {
    r#match::from_extension(ext)
}
