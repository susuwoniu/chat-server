use headers::{Header, HeaderName, HeaderValue};
use once_cell::sync::Lazy;
static X_CLIENT_ID: Lazy<HeaderName> = Lazy::new(|| HeaderName::from_static("x-client-id"));
#[derive(Debug)]
pub struct XClientId(pub i64);

impl Header for XClientId {
  fn name() -> &'static HeaderName {
    &X_CLIENT_ID
  }

  fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
  where
    I: Iterator<Item = &'i HeaderValue>,
  {
    let value = values.next().ok_or_else(headers::Error::invalid)?;
    // todo i64
    let x_client_id: i64 = value
      .to_str()
      .map_err(|_| headers::Error::invalid())?
      .parse()
      .map_err(|_| headers::Error::invalid())?;
    Ok(XClientId(x_client_id))
  }

  fn encode<E>(&self, values: &mut E)
  where
    E: Extend<HeaderValue>,
  {
    let value = HeaderValue::from_str(self.0.to_string().as_str());

    values.extend(value);
  }
}
