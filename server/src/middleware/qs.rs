use crate::{error::ServiceError, middleware::Locale};
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts},
};
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
    let value = serde_qs::from_str(query)
      .map_err(|err| ServiceError::param_invalid(&locale, "parse_query_failed", err.into()))?;
    Ok(Qs(value))
  }
}

impl<T> Deref for Qs<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::extract::RequestParts;
  use axum::http::Request;
  use serde::Deserialize;
  use std::fmt::Debug;

  async fn check<T: DeserializeOwned + PartialEq + Debug>(uri: impl AsRef<str>, value: T) {
    let mut req = RequestParts::new(Request::builder().uri(uri.as_ref()).body(()).unwrap());
    assert_eq!(Qs::<T>::from_request(&mut req).await.unwrap().0, value);
  }

  #[tokio::test]
  async fn test_query() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Pagination {
      size: Option<u64>,
      page: Option<u64>,
    }

    check(
      "http://example.com/test",
      Pagination {
        size: None,
        page: None,
      },
    )
    .await;

    check(
      "http://example.com/test?size=10",
      Pagination {
        size: Some(10),
        page: None,
      },
    )
    .await;

    check(
      "http://example.com/test?size=10&page=20",
      Pagination {
        size: Some(10),
        page: Some(20),
      },
    )
    .await;
  }
}
