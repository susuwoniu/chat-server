use crate::{
  account::constant::{ACCESS_TOKEN_KEY, PHONE_AUTH_CODE_TEMP_KEY},
  global::{AccessTokenPair, Config, RefreshTokenPair},
  util::{datetime_tz, id::next_id, key_pair::Pair, string_i64},
};
use chrono::{Duration, NaiveDateTime, Utc};
use pasetors::claims::Claims;
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey, Version};
use pasetors::public;
use serde::{Deserialize, Serialize};
pub fn get_phone_code_temp_key(phone_country_code: &i32, phone_number: &str) -> String {
  let temp_key = format!(
    "{}/{}{}",
    PHONE_AUTH_CODE_TEMP_KEY, phone_country_code, phone_number
  );
  return temp_key;
}
pub fn get_access_token_key(token: String) -> String {
  let temp_key = format!("{}/{}", ACCESS_TOKEN_KEY, token);
  return temp_key;
}

pub struct Token {
  token: String,
  expires_at: NaiveDateTime,
}

impl Token {
  pub fn new(
    account_id: &i64,
    pair: &Pair,
    expires_in_minutes: i64,
    issuer: String,
    audience: String,
    client_id: &i64,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthData {
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub access_token: String,
  #[serde(with = "datetime_tz")]
  pub expires_at: NaiveDateTime,
  pub refresh_token: String,
  #[serde(with = "datetime_tz")]
  pub refresh_token_expires_at: NaiveDateTime,
}
impl AuthData {
  pub fn new(account_id: &i64, client_id: &i64) -> Self {
    let config = Config::global();
    let token = Token::new(
      account_id,
      &AccessTokenPair::global().0,
      config.auth.access_token_expires_in_minutes,
      config.server.url.clone().into(),
      config.server.url.clone().into(),
      client_id,
    );
    let access_token = token.get_token();

    let refresh = Token::new(
      account_id,
      &RefreshTokenPair::global().0,
      config.auth.refresh_token_expires_in_days,
      config.server.url.clone().into(),
      config.server.url.clone().into(),
      client_id,
    );
    let refresh_token = refresh.get_token();
    let expires_at = token.get_expires_at();
    Self {
      account_id: *account_id,
      access_token,
      expires_at: expires_at,
      refresh_token,
      refresh_token_expires_at: refresh.get_expires_at(),
    }
  }
}
