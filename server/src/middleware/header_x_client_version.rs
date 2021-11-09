use headers::{Header, HeaderName, HeaderValue};
use once_cell::sync::Lazy;
static X_CLIEN_VERSION: Lazy<HeaderName> =
  Lazy::new(|| HeaderName::from_static("x-client-version"));
#[derive(Debug)]
pub struct XClientVersion(pub String);
impl Header for XClientVersion {
  fn name() -> &'static HeaderName {
    &X_CLIEN_VERSION
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
    Ok(XClientVersion(header_value))
  }

  fn encode<E>(&self, values: &mut E)
  where
    E: Extend<HeaderValue>,
  {
    let value = HeaderValue::from_str(self.0.to_string().as_str());

    values.extend(value);
  }
}
