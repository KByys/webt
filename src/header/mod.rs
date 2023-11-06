pub mod authorization;
pub mod content;
pub mod cookie;
use std::{fmt::Display, string::FromUtf8Error};

pub use authorization::Bearer;
use base64::DecodeError;
pub use content::ContentType;
use hyper::header::ToStrError;
use hyper::header::HeaderName;

pub struct Header<T>(pub T);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderParserError {
    InvalidValue(String),
    InvalidUtf8String,
    MissingHeaderValue(HeaderName),
    Base64DecodeError(DecodeError),
    NotAllVisibleAscii,
}
impl Display for HeaderParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderParserError::InvalidUtf8String => f.write_str("Invalid UTF-8 string"),
            HeaderParserError::InvalidValue(value) => {
                f.write_fmt(format_args!("{} is not a valid value", value))
            }
            HeaderParserError::MissingHeaderValue(value) => {
                f.write_fmt(format_args!("{} header value missing", value))
            }
            HeaderParserError::Base64DecodeError(e) => Display::fmt(&e, f),
            HeaderParserError::NotAllVisibleAscii => f.write_str("Not all visible ASCII chars"),
        }
    }
}
impl std::error::Error for HeaderParserError {}
impl From<FromUtf8Error> for HeaderParserError {
    fn from(_value: FromUtf8Error) -> Self {
        Self::InvalidUtf8String
    }
}
impl From<ToStrError> for HeaderParserError {
    fn from(_value: ToStrError) -> Self {
        Self::NotAllVisibleAscii
    }
}