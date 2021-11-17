use headers::{Header, HeaderName, HeaderValue};
use once_cell::sync::Lazy;
static X_CLIENT_PLATFORM: Lazy<HeaderName> =
  Lazy::new(|| HeaderName::from_static("x-client-platform"));
#[derive(Debug)]
pub struct XClientPlatform(pub String);
impl Header for XClientPlatform {
  fn name() -> &'static HeaderName {
    &X_CLIENT_PLATFORM
  }

  fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
  where
    I: Iterator<Item = &'i HeaderValue>,
  {
    let value = values.next().ok_or_else(headers::Error::invalid)?;
    let header_value: String = value
      .to_str()
      .map_err(|_| headers::Error::invalid())?
      .to_string();
    return Ok(XClientPlatform(header_value));
  }

  fn encode<E>(&self, values: &mut E)
  where
    E: Extend<HeaderValue>,
  {
    let value = HeaderValue::from_str(self.0.to_string().as_str());

    values.extend(value);
  }
}
