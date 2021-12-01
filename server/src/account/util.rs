use crate::{
  account::model::{AuthData, FullAccount, TokenType},
  constant::{PHONE_AUTH_CODE_TEMP_KEY, REFRESH_TOKEN_KEY},
  global::{AccessTokenPair, Config, RefreshTokenPair},
  types::Action,
  util::{id::next_id, key_pair::Pair},
};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use pasetors::claims::Claims;
use pasetors::keys::{AsymmetricPublicKey, AsymmetricSecretKey, Version};
use pasetors::public;

pub fn get_phone_code_temp_key(
  phone_country_code: &i32,
  phone_number: &str,
  device_id: &str,
) -> String {
  let temp_key = format!(
    "{}/{}{}/{}",
    PHONE_AUTH_CODE_TEMP_KEY, phone_country_code, phone_number, device_id
  );
  return temp_key;
}
pub fn get_refresh_token_key(account_id: i64, device_id: &str) -> String {
  let temp_key = format!("{}/{}/{}", REFRESH_TOKEN_KEY, account_id, device_id);
  return temp_key;
}

pub struct Token {
  pub token: String,
  pub expires_at: NaiveDateTime,
  pub jti: i64,
}

impl Token {
  pub fn new(
    account_id: &i64,
    pair: &Pair,
    expires_in_minutes: i64,
    issuer: String,
    audience: String,
    client_id: &i64,
    roles: Vec<String>,
    device_id: String,
    now: DateTime<Utc>,
  ) -> Token {
    let mut claims = Claims::new().expect("new claims failed");
    claims.issuer(&issuer).expect("get issuer failed");
    claims
      .subject(&account_id.to_string())
      .expect("subject failed");
    claims.audience(&audience).expect("get audience failed");
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
      .add_additional("device_id", device_id.to_string())
      .expect("get device_id failed");
    claims
      .token_identifier(&jti.to_string())
      .expect("get jti failed");
    claims
      .add_additional("roles", roles.join(","))
      .expect("get roles failed");
    let sk = AsymmetricSecretKey::from(&pair.get_secret_bytes(), Version::V4)
      .expect("get secret key failed");
    let pk = AsymmetricPublicKey::from(&pair.get_public_bytes(), Version::V4)
      .expect("get public key failed");
    let pub_token = public::sign(&sk, &pk, &claims, None, None).expect("get token failed");
    Token {
      token: pub_token,
      jti: jti,
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

impl AuthData {
  pub fn new(
    account_id: &i64,
    client_id: &i64,
    device_id: String,
    roles: Vec<String>,
    actions: Vec<Action>,
    account: FullAccount,
    now: DateTime<Utc>,
  ) -> Self {
    let config = Config::global();
    let token = Token::new(
      account_id,
      &AccessTokenPair::global().0,
      config.auth.access_token_expires_in_minutes,
      config.server.url.clone().into(),
      config.server.url.clone().into(),
      client_id,
      roles.clone(),
      device_id.clone(),
      now,
    );
    let access_token = token.get_token();

    let refresh = Token::new(
      account_id,
      &RefreshTokenPair::global().0,
      config.auth.refresh_token_expires_in_days * 24 * 60,
      config.server.url.clone().into(),
      config.server.url.clone().into(),
      client_id,
      roles.clone(),
      device_id.clone(),
      now,
    );
    let refresh_token = refresh.get_token();
    let expires_at = token.get_expires_at();
    Self {
      account_id: *account_id,
      access_token,
      id: token.jti,
      access_token_type: TokenType::Bearer,
      access_token_expires_at: expires_at,
      refresh_token,
      refresh_token_id: refresh.jti,
      refresh_token_type: TokenType::Bearer,
      refresh_token_expires_at: refresh.get_expires_at(),
      device_id: device_id,
      actions,
      account,
      im_username: format!("im{}", account_id),
      im_access_token: "".to_string(),
      im_access_token_expires_at: NaiveDateTime::from_timestamp(0, 0),
    }
  }
  pub fn im_access_token_mut(&mut self) -> &mut String {
    &mut self.im_access_token
  }
  pub fn im_access_token_expires_at_mut(&mut self) -> &mut NaiveDateTime {
    &mut self.im_access_token_expires_at
  }
  pub fn set_im_token(
    mut self,
    im_access_token: String,
    im_access_token_expires_at: NaiveDateTime,
  ) -> Self {
    *self.im_access_token_mut() = im_access_token;
    *self.im_access_token_expires_at_mut() = im_access_token_expires_at;
    self
  }
}
