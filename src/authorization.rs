#[cfg(feature = "axum")]
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Bearer {
    type Rejection = String;
    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth) = parts.headers.get("Authorization") {
            match auth.to_str() {
                Ok(bearer) => Ok(Bearer {
                    bearer: Some(bearer.to_string()),
                }),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Ok(Bearer { bearer: None })
        }
    }
}

pub struct Bearer {
    bearer: Option<String>,
}

impl Bearer {
    pub fn token(&self) -> Option<&str> {
        self.bearer.as_ref()?.strip_prefix("Bearer ")
    }
}
