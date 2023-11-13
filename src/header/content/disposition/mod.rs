use std::collections::HashMap;

use hyper::header::HeaderValue;
use hyper::HeaderMap;

use crate::byte_serialize;
use crate::header::{HeaderKey, HeaderParserError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentDisposition {
    inner: String,
    filename: Option<String>,
    name: Option<String>,
}
impl Default for ContentDisposition {
    fn default() -> Self {
        Self {
            inner: "inline".to_owned(),
            filename: None,
            name: None,
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

fn parse(inner: &str) -> (Option<String>, Option<String>) {
    if let Some(value) = inner.strip_prefix("attachment;") {
        let decoded_str: HashMap<String, String> = url::form_urlencoded::parse(value.as_bytes())
            .map(|f| (f.0.trim().into(), f.1.trim().into()))
            .collect();
        let filename = decoded_str
            .get("filename")
            .map(|name| name.replace('\"', ""));
        let name = decoded_str.get("name").map(|name| name.replace('\"', ""));
        (filename, name)
    } else {
        (None, None)
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
            let (filename, name) = parse(value);
            Ok(Self {
                inner: value.trim().into(),
                filename,
                name,
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
        Some(self.filename.as_ref()?)
    }
    pub fn name(&self) -> Option<&str> {
        Some(self.name.as_ref()?)
    }

    pub fn is_inline(&self) -> bool {
        self.inner.starts_with("inline")
    }
    pub fn new(filename: Option<String>, name: Option<String>) -> Self {
        let mut encode_str = String::new();
        if let Some(filename) = &filename {
            encode_str = format!("filename=\"{}\"", byte_serialize(filename.as_bytes()));
        }

        if let Some(name) = &name {
            if encode_str.is_empty() {
                encode_str = format!("name=\"{}\"", byte_serialize(name.as_bytes()));
            } else {
                encode_str = format!("{}&name=\"{}\"", encode_str, name);
            }
        }
        if encode_str.is_empty() {
            Self {
                inner: "attachment;".to_owned(),
                filename: None,
                name: None,
            }
        } else {
            Self {
                inner: format!("attachment; {}", encode_str),
                filename,
                name,
            }
        }
    }
    pub fn new_with_filename(filename: impl AsRef<str>) -> Self {
        let inner = format!(
            "filename=\"{}\"",
            byte_serialize(filename.as_ref().as_bytes())
        );
        Self {
            inner: format!("attachment; {}", inner),
            filename: Some(filename.as_ref().into()),
            name: None,
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
