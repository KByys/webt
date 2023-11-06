use hyper::header::{HeaderMap, HeaderValue};

use crate::header::HeaderParserError;
// #[cfg(feature = "axum")]
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Bearer {
    type Rejection = String;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Bearer::try_from(&parts.headers).map_err(|err|err.to_string())
    }
}
use hyper::header::AUTHORIZATION;
impl TryFrom<&HeaderMap> for Bearer {
    type Error = HeaderParserError;

    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        match value.get(AUTHORIZATION) {
            Some(value) => Self::try_from(value),
            _ => Err(HeaderParserError::MissingHeaderValue(AUTHORIZATION)),
        }
    }
}
impl TryFrom<&HeaderValue> for Bearer {
    type Error = HeaderParserError;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        match value.to_str() {
            Ok(value) => Self::try_from(value),
            _ => Err(HeaderParserError::NotAllVisibleAscii),
        }
    }
}

impl TryFrom<String> for Bearer {
    type Error = HeaderParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl TryFrom<&String> for Bearer {
    type Error = HeaderParserError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Bearer {
    type Error = HeaderParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(token) = value.strip_prefix("Bearer ") {
            Ok(Self {
                token: token.into(),
            })
        } else {
            Err(HeaderParserError::InvalidValue(value.into()))
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bearer {
    token: String,
}
impl AsRef<str> for Bearer {
    fn as_ref(&self) -> &str {
        &self.token
    }
}
impl Bearer {
    pub fn token(&self) -> &str {
        self.token.as_str()
    }
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
    pub fn bearer(&self) -> String {
        format!("Bearer {}", self.token)
    }
}
