use crate::{
  error::ServiceError, global::Config, middleware::header_x_forwarded_for::XForwardedFor,
  middleware::header_x_real_ip::XRealIp,
};
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts, TypedHeader},
};
#[derive(Debug, Clone)]
pub struct Ip(pub String);
impl Ip {
  pub fn new(ip: &str) -> Self {
    Ip(ip.to_string())
  }
}

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
    let mut header_forwarded_for_value_string: String = "".to_string();
    if let Ok(TypedHeader(XRealIp(header_value))) = header_value {
      // parse language
      header_value_string = header_value;
      dbg!(&header_value_string);
    }
    if let Ok(TypedHeader(XForwardedFor(header_forwarded_for_value))) = header_forwarded_for_value {
      // parse language
      header_forwarded_for_value_string = header_forwarded_for_value;
      dbg!(&header_forwarded_for_value_string);
    }
    Ok(Ip(header_value_string))
  }
}
