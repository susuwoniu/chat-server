use headers::{Header, HeaderName, HeaderValue};
use once_cell::sync::Lazy;
static X_CLIENT_SIGNATURE: Lazy<HeaderName> =
  Lazy::new(|| HeaderName::from_static("x-client-signature"));
#[derive(Debug)]
pub struct XClientSignature(pub String);

impl Header for XClientSignature {
  fn name() -> &'static HeaderName {
    &X_CLIENT_SIGNATURE
  }

  fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
  where
    I: Iterator<Item = &'i HeaderValue>,
  {
    let value = values.next().ok_or_else(headers::Error::invalid)?;
    let x_client_signature: String = value
      .to_str()
      .map_err(|_| headers::Error::invalid())?
      .to_string();
    Ok(XClientSignature(x_client_signature))
  }

  fn encode<E>(&self, values: &mut E)
  where
    E: Extend<HeaderValue>,
  {
    let value = HeaderValue::from_str(self.0.to_string().as_str());
    values.extend(value);
  }
}
