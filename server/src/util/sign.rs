use crate::error::ServiceError;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
pub type HmacSha256 = Hmac<Sha256>;
use subtle_encoding::hex;

pub struct Sign {
  method: String,
  path: String,
  query: String,
  content_type: String,
  client_id: String,
  client_date: String,
  body: Option<String>,
}

/* postman js code generate

const client_id = "";
const client_secret = "";
const now = new Date().toISOString();
const body  = pm.request.body.raw;

pm.request.headers.add({key: 'x-client-id', value: client_id });
pm.request.headers.add({key: 'x-client-date', value: now });

const hmacDigest = CryptoJS.HmacSHA256(body, client_secret);
const body_hex = CryptoJS.enc.Hex.stringify(hmacDigest);

const plain_text = `${pm.request.method}\n${pm.request.url.getPath()}\n${pm.request.url.getQueryString()}\ncontent-type=application/json&x-client-id=${client_id}&x-client-date=${now}\n${body_hex}`
const signhmacDigest = CryptoJS.HmacSHA256(plain_text, client_secret);
const signature = "v1.signature."+CryptoJS.enc.Hex.stringify(signhmacDigest);
pm.request.headers.add({
    key:"x-client-signature",
    value:signature
})
*/

/*
CanonicalRequest =
  HTTPRequestMethod + '\n' +
  CanonicalURI + '\n' +
  CanonicalQueryString + '\n' +
  CanonicalHeaders + '\n' +
  HexEncode(Hash(RequestPayload))
See https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html

POST
/api/v1/accounts/phone-code

content-type=application/json&x-client-date=202123414231&x-client-id=134124231424234
hash(body)

x-client-signature=v1.signature.<hash>

*/
impl Sign {
  pub fn new(
    method: String,
    path: String,
    query: String,
    content_type: String,
    client_id: String,
    client_date: String,
    body: Option<String>,
  ) -> Self {
    Sign {
      method,
      path,
      query,
      content_type,
      client_id,
      client_date,
      body,
    }
  }
  pub fn get_sinature(&self, secret: String) -> String {
    let header_ensure = format!(
      "content-type={}&x-client-id={}&x-client-date={}",
      self.content_type, self.client_id, self.client_date
    );
    // Create alias for HMAC-SHA256

    let mut body_hex = None;

    if self.body.is_some() {
      // Create HMAC-SHA256 instance which implements `Mac` trait
      body_hex = Some(hmac_sha256(
        self.body.clone().unwrap().as_str(),
        secret.clone(),
      ));
    }

    let mut body_ensutre = "".to_string();
    if body_hex.is_some() {
      body_ensutre = body_hex.unwrap();
    }

    let plain_string = format!(
      "{}\n{}\n{}\n{}\n{}",
      self.method, self.path, self.query, header_ensure, body_ensutre
    );
    let signature = format!("v1.signature.{}", hmac_sha256(&plain_string, secret));
    signature
  }
  pub fn verify(&self, hex_code: String, secret: String) -> Result<bool, ServiceError> {
    // verify time

    // signature_client_date_expires_in_minutes
    let client_date: DateTime<Utc> =
      DateTime::from(DateTime::parse_from_rfc3339(&self.client_date)?);
    let now = Utc::now();
    let offset = now - client_date;
    let offset_minutes = offset.num_minutes().abs();
    if offset_minutes > 10 {
      return Err(ServiceError::Unauthorized(
        "x-client-date is not match server time".to_string(),
      ));
    }
    let signature = self.get_sinature(secret);
    if signature == hex_code {
      Ok(true)
    } else {
      Err(ServiceError::Unauthorized(
        "x-client-signature error".to_string(),
      ))
    }
    // calculate
  }
}

pub fn hmac_sha256(input: &str, secret: String) -> String {
  let mut mac =
    HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
  mac.update(input.as_bytes());
  // `result` has type `Output` which is a thin wrapper around array of
  // bytes for providing constant time equality check
  let result = mac.finalize();
  // To get underlying array use `into_bytes` method, but be careful, since
  // incorrect use of the code value may permit timing attacks which defeat
  // the security provided by the `Output`
  let code_bytes = result.into_bytes();
  let code_hex = hex::encode(&code_bytes);
  let code = String::from_utf8(code_hex).expect("parse hex code failed");
  return code;
}
