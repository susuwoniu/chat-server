use crate::config::Config;
use crate::util::key_pair::Pair;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorBadRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config as BearerConfig};
use actix_web_httpauth::extractors::AuthenticationError;
use futures::future::{err, ok, Ready};
use pasetors::claims::ClaimsValidationRules;
use pasetors::keys::{AsymmetricPublicKey, Version};
use pasetors::public;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct AuthToken {
  pub account_id: i64,
  pub token: String,
  pub client_id: i64,
}
pub fn validate_token(token: &str, pair: &Pair, config: &Config) -> Result<AuthToken, Error> {
  dbg!(token);
  let mut validation_rules = ClaimsValidationRules::new();
  validation_rules.validate_issuer_with(config.server.url.as_str());
  validation_rules.validate_audience_with(config.server.url.as_str());
  let pk = AsymmetricPublicKey::from(&pair.get_public_bytes(), Version::V4)
    .expect("get public key failed");
  let verify_result = public::verify(&pk, token, &validation_rules, None, None);
  // verify claim
  // create meta

  match verify_result {
    Ok(result) => {
      dbg!(&result);
      let sub = result.get_claim("sub").expect("get sub failed");
      let azp = result.get_claim("azp").expect("get azp failed");
      let auth = AuthToken {
        account_id: i64::from_str(sub.as_str().expect("parse sub failed"))
          .expect("parse sub failed"),
        token: token.to_string(),
        client_id: i64::from_str(azp.as_str().expect("parse sub failed"))
          .expect("parse sub failed"),
      };
      Ok(auth)
    }
    Err(_) => Err(ErrorUnauthorized("token verify failed")),
  }
}

pub async fn validator(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
  let bearer_config = req
    .app_data::<BearerConfig>()
    .map(|data| data.clone())
    .unwrap_or_else(Default::default);
  let pair = req.app_data::<web::Data<Pair>>().expect("get pair failed");
  let config = req
    .app_data::<web::Data<Config>>()
    .expect("get config failed");

  match validate_token(credentials.token(), pair.as_ref(), &config) {
    Ok(res) => {
      dbg!(&res);
      req.head().extensions_mut().insert(res);
      Ok(req)
    }
    Err(_) => Err(AuthenticationError::from(bearer_config).into()),
  }
}
impl FromRequest for AuthToken {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  type Config = ();

  fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
    if let Some(AuthToken {
      account_id,
      token,
      client_id,
    }) = req.extensions().get::<AuthToken>()
    {
      ok(AuthToken {
        account_id: *account_id,
        token: token.clone(),
        client_id: *client_id,
      })
    } else {
      err(ErrorBadRequest("locale is missing"))
    }
  }
}
