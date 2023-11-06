use hyper::header::HeaderValue;
use hyper::HeaderMap;
use url;
use url::form_urlencoded::byte_serialize;

use crate::header::{HeaderKey, HeaderParserError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentDisposition {
    inner: String,
    file_name: Option<String>,
}
impl Default for ContentDisposition {
    fn default() -> Self {
        Self {
            inner: "inline".to_owned(),
            file_name: None,
        }
    }
}
use hyper::header::CONTENT_DISPOSITION;
impl TryFrom<&HeaderMap> for ContentDisposition {
    type Error = HeaderParserError;

    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        match value.get(CONTENT_DISPOSITION) {
            Some(value) => Self::try_from(value),
            _ => Err(HeaderParserError::MissingHeaderValue(CONTENT_DISPOSITION)),
        }
    }
}
impl TryFrom<&HeaderValue> for ContentDisposition {
    type Error = HeaderParserError;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        Self::try_from(value.to_str()?)
    }
}

fn parse_file_name(inner: &str) -> Option<String> {
    let decoded_str: Vec<_> = url::form_urlencoded::parse(inner.as_bytes())
        .map(|f| f.1)
        .collect();
    let file_name = decoded_str.get(0)?;
    if file_name.is_empty() {
        None
    } else {
        Some(file_name.trim().replace('\"', ""))
    }
}
impl TryFrom<String> for ContentDisposition {
    type Error = HeaderParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for ContentDisposition {
    type Error = HeaderParserError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl TryFrom<&str> for ContentDisposition {
    type Error = HeaderParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("inline") || value.starts_with("attachment") {
            Ok(Self {
                inner: value.trim().into(),
                file_name: parse_file_name(value.trim()),
            })
        } else {
            Err(HeaderParserError::InvalidValue(value.into()))
        }
    }
}
use hyper::http::Error;
impl TryFrom<ContentDisposition> for HeaderValue {
    type Error = Error;

    fn try_from(value: ContentDisposition) -> Result<Self, Self::Error> {
        Ok(HeaderValue::from_str(&value.inner)?)
    }
}

impl ContentDisposition {
    pub fn file_name(&self) -> Option<&str> {
        Some(self.file_name.as_ref()?)
    }
    pub fn is_inline(&self) -> bool {
        self.inner.starts_with("inline")
    }
    pub fn new(file_name: impl AsRef<str>) -> Self {
        let encode_file_name: String = byte_serialize(file_name.as_ref().as_bytes()).collect();
        Self {
            inner: format!("attachment; filename=\"{}\"", encode_file_name),
            file_name: Some(file_name.as_ref().into()),
        }
    }
}

impl HeaderKey for ContentDisposition {
    fn header_name(&self) -> hyper::http::HeaderName {
        CONTENT_DISPOSITION
    }

    fn value(&self) -> &str {
        &self.inner
    }
}
