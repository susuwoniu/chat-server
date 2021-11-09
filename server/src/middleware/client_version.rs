use crate::{
  error::{Error, ServiceError},
  middleware::header_x_client_version::XClientVersion,
  middleware::Locale,
};
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts, TypedHeader},
};
use semver::Version;
#[derive(Debug, Clone)]
pub struct ClientVersion(pub Version);

#[async_trait]
impl<B> FromRequest<B> for ClientVersion
where
  B: Send,
{
  type Rejection = ServiceError;
  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let header_value = TypedHeader::<XClientVersion>::from_request(req).await;
    let mut header_value_string: String = "".to_string();
    let locale = Locale::from_request(req).await?;

    if let Ok(TypedHeader(XClientVersion(header_value))) = header_value {
      // parse language
      header_value_string = header_value;
    }
    if !header_value_string.is_empty() {
      let parse_result = Version::parse(&header_value_string);
      match parse_result {
        Ok(value) => Ok(ClientVersion(value)),
        Err(err) => Err(ServiceError::bad_request(
          &locale,
          "can_not_parse_client_version",
          err.into(),
        )),
      }
    } else {
      Err(ServiceError::bad_request(
        &locale,
        "can_not_found_header_client_version",
        Error::Default,
      ))
    }
  }
}
