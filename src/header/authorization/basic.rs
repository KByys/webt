use base64::{engine::general_purpose::STANDARD, Engine};
use hyper::header::{HeaderMap, HeaderValue};

use crate::header::{HeaderKey, HeaderParserError};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Basic {
    basic: String,
    username: String,
    password: String,
}

#[cfg(feature = "axum")]
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Basic {
    type Rejection = String;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Self::try_from(&parts.headers).map_err(|err| err.to_string())
    }
}

use hyper::header::AUTHORIZATION;
impl TryFrom<&HeaderMap> for Basic {
    type Error = HeaderParserError;
    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        match value.get(AUTHORIZATION) {
            Some(value) => Self::try_from(value),
            _ => Err(HeaderParserError::MissingHeaderValue(AUTHORIZATION)),
        }
    }
}
impl TryFrom<&HeaderValue> for Basic {
    type Error = HeaderParserError;
    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        Self::try_from(value.to_str()?)
    }
}

impl TryFrom<String> for Basic {
    type Error = HeaderParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl TryFrom<&String> for Basic {
    type Error = HeaderParserError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Basic {
    type Error = HeaderParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match value.strip_prefix("Basic ") {
            Some(value) => value,
            _ => value,
        };
        let decoded_str = match STANDARD.decode(value) {
            Ok(value) => String::from_utf8(value)?,
            Err(e) => return Err(HeaderParserError::Base64DecodeError(e)),
        };

        let basic: Vec<_> = decoded_str.splitn(2, ':').collect();
        if basic.len() == 2 {
            Ok(Self {
                basic: decoded_str.to_string(),
                username: basic[0].trim().to_owned(),
                password: basic[1].trim().to_owned(),
            })
        } else {
            Err(HeaderParserError::InvalidValue(decoded_str))
        }
    }
}

impl Basic {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        let username = username.into();
        let password = password.into();
        let basic = STANDARD.encode(format!("{}:{}", username, password).as_bytes());
        Self {
            basic,
            username,
            password,
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
impl HeaderKey for Basic {
    fn header_name(&self) -> hyper::http::HeaderName {
        AUTHORIZATION
    }

    fn value(&self) -> &str {
        &self.basic
    }
}
