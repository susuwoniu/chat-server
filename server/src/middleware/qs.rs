use crate::{error::ServiceError, middleware::Locale};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use ruma_serde::urlencoded;
use serde::de::DeserializeOwned;
use std::ops::Deref;
#[derive(Debug, Clone, Copy, Default)]
pub struct Qs<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Qs<T>
where
    T: DeserializeOwned,
    B: Send,
{
    type Rejection = ServiceError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let locale = Locale::from_request(req).await?;
        let query = req.uri().query().unwrap_or_default();
        let value = urlencoded::from_str(query).map_err(|err| {
            ServiceError::param_invalid(&locale, "parse_query_failed", err.into())
        })?;
        Ok(Qs(value))
    }
}

impl<T> Deref for Qs<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
