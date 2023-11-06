mod basic;
mod bearer;
pub use basic::Basic;
pub use bearer::Bearer;

use crate::Header;

// #[cfg(features = "axum")]
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Header<Option<Basic>> {
    type Rejection = String;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Header(Basic::try_from(&parts.headers).ok()))
    }
}
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Header<Option<Bearer>> {
    type Rejection = String;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Header(Bearer::try_from(&parts.headers).ok()))
    }
}
