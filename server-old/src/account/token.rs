use crate::util::id::next_id;
use crate::util::key_pair::Pair;
use chrono::{Duration, NaiveDateTime, Utc};
use pasetors::claims::Claims;
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey, Version};
use pasetors::public;
pub struct Token {
  token: String,
  expires_at: NaiveDateTime,
}

impl Token {
  pub fn new(
    account_id: i64,
    pair: &Pair,
    expires_in_minutes: i64,
    issuer: String,
    audience: String,
    client_id: i64,
  ) -> Token {
    let mut claims = Claims::new().expect("new claims failed");
    claims.issuer(&issuer).expect("get issuer failed");
    claims
      .subject(&account_id.to_string())
      .expect("subject failed");
    claims.audience(&audience).expect("get audience failed");
    let now = Utc::now();
    let iat = now;
    let nbf = iat;
    let expires_at = now + Duration::minutes(expires_in_minutes);
    let jti = next_id();
    claims.issued_at(&iat.to_rfc3339()).expect("get iat failed");
    claims
      .not_before(&nbf.to_rfc3339())
      .expect("get nbf failed");
    claims
      .expiration(&expires_at.to_rfc3339())
      .expect("get expires at error");
    // add client id for token issueed to which client
    // azp
    claims
      .add_additional("client_id", client_id.to_string())
      .expect("get client_id failed");
    claims
      .token_identifier(&jti.to_string())
      .expect("get jti failed");
    let sk = AsymmetricSecretKey::from(&pair.get_secret_bytes(), Version::V4)
      .expect("get secret key failed");
    let pk = AsymmetricPublicKey::from(&pair.get_public_bytes(), Version::V4)
      .expect("get public key failed");
    let pub_token = public::sign(&sk, &pk, &claims, None, None).expect("get token failed");
    Token {
      token: pub_token,
      expires_at: expires_at.naive_utc(),
    }
  }
  pub fn get_token(&self) -> String {
    return self.token.clone();
  }
  pub fn get_expires_at(&self) -> NaiveDateTime {
    return self.expires_at.clone();
  }
}
