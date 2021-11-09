use headers::{Header, HeaderName, HeaderValue};
use once_cell::sync::Lazy;
static X_REAL_IP: Lazy<HeaderName> = Lazy::new(|| HeaderName::from_static("x-real-ip"));
#[derive(Debug)]
pub struct XRealIp(pub String);

impl Header for XRealIp {
  fn name() -> &'static HeaderName {
    &X_REAL_IP
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
    Ok(XRealIp(header_value))
  }

  fn encode<E>(&self, values: &mut E)
  where
    E: Extend<HeaderValue>,
  {
    let value = HeaderValue::from_str(self.0.to_string().as_str());
    values.extend(value);
  }
}
