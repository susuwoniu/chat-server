use crate::{
  error::{Error, ServiceError},
  middleware::header_x_forwarded_for::XForwardedFor,
  middleware::header_x_real_ip::XRealIp,
  middleware::Locale,
};
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts, TypedHeader},
};
use ipnetwork17::IpNetwork;
#[derive(Debug, Clone)]
pub struct Ip(pub IpNetwork);

#[async_trait]
impl<B> FromRequest<B> for Ip
where
  B: Send,
{
  type Rejection = ServiceError;
  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let header_value = TypedHeader::<XRealIp>::from_request(req).await;
    let mut header_value_string: String = "".to_string();

    let header_forwarded_for_value = TypedHeader::<XForwardedFor>::from_request(req).await;
    let locale = Locale::from_request(req).await?;

    if let Ok(TypedHeader(XRealIp(header_value))) = header_value {
      // parse language
      header_value_string = header_value;
    } else if let Ok(TypedHeader(XForwardedFor(header_forwarded_for_value))) =
      header_forwarded_for_value
    {
      // parse language
      header_value_string = header_forwarded_for_value;
    }
    if !header_value_string.is_empty() {
      let ip_parse_result = header_value_string.parse::<IpNetwork>();
      match ip_parse_result {
        Ok(ip_network) => Ok(Ip(ip_network)),
        Err(err) => Err(ServiceError::bad_request(
          &locale,
          "can_not_parse_ip",
          err.into(),
        )),
      }
    } else {
      // default 127.0.0.1
      Ok(Ip("127.0.0.1".parse::<IpNetwork>().unwrap()))
    }
  }
}
