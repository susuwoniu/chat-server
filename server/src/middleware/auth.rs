use crate::{
  error::{Error, ServiceError},
  global::{AccessTokenPair, Config},
  middleware::{header_x_client_id::XClientId, Locale},
};
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts, TypedHeader},
};
use headers::{authorization::Bearer, Authorization};
use pasetors::claims::ClaimsValidationRules;
use pasetors::keys::{AsymmetricPublicKey, Version};
use pasetors::public;
use std::str::FromStr;
#[derive(Debug)]
pub struct Auth {
  pub account_id: i64,
  pub client_id: i64,
}
#[async_trait]
impl<B> FromRequest<B> for Auth
where
  B: Send,
{
  type Rejection = ServiceError;

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    // get locale
    let locale = Locale::from_request(req).await?;
    let cfg = Config::global();
    let TypedHeader(Authorization(bearer)) =
      TypedHeader::<Authorization<Bearer>>::from_request(req)
        .await
        .map_err(|err| {
          ServiceError::unauthorized(&locale, "parse_header_authorization_failed", err.into())
        })?;
    let TypedHeader(XClientId(x_client_id)) = TypedHeader::<XClientId>::from_request(req).await?;
    let mut validation_rules = ClaimsValidationRules::new();
    validation_rules.validate_issuer_with(cfg.server.url.as_str());
    validation_rules.validate_audience_with(cfg.server.url.as_str());
    let access_token_pair = &AccessTokenPair::global().0;
    let pk = AsymmetricPublicKey::from(&access_token_pair.get_public_bytes(), Version::V4)
      .expect("get public key failed");

    let verify_result = public::verify(&pk, bearer.token(), &validation_rules, None, None);
    match verify_result {
      Ok(result) => {
        dbg!(&result);
        let sub_option = result.get_claim("sub");
        let client_id_option = result.get_claim("client_id");
        if let (Some(sub), Some(client_id)) = (sub_option, client_id_option) {
          let client_id = client_id.as_str();
          if let Some(client_id) = client_id {
            let client_id = i64::from_str(client_id).map_err(|_| {
              ServiceError::unauthorized(
                &locale,
                "token_client_id_invalid",
                Error::Other(format!(
                  "client_id can not parse to i64, token: {} , client_id: {:?}, x-client-id: {},invalid",
                  bearer.token(),
                  client_id,
                  x_client_id
                )),
              )
            })?;
            if client_id == x_client_id {
              let sub = sub.as_str();
              if let Some(sub) = sub {
                let sub = i64::from_str(sub).map_err(|_| {
                  ServiceError::unauthorized(
                    &locale,
                    "token_sub_invalid",
                    Error::Other(format!("token: {} , sub: {:?}", bearer.token(), sub)),
                  )
                })?;

                // verified
                let auth = Auth {
                  account_id: sub,
                  client_id,
                };
                return Ok(auth);
              } else {
                return Err(ServiceError::unauthorized(
                  &locale,
                  "token_sub_invalid",
                  Error::Other(format!("token: {} , sub: {:?}", bearer.token(), sub)),
                ));
              }
            } else {
              return Err(ServiceError::unauthorized(
                &locale,
                "token_client_id_invalid",
                Error::Other(format!(
                  "token: {} , client_id: {}, x-client-id: {},invalid",
                  bearer.token(),
                  client_id,
                  x_client_id
                )),
              ));
            }
          } else {
            return Err(ServiceError::unauthorized(
              &locale,
              "token_client_id_invalid",
              Error::Other(format!(
                "token: {} , client_id: {:?}, x-client-id: {},invalid",
                bearer.token(),
                client_id,
                x_client_id
              )),
            ));
          }
        } else {
          return Err(ServiceError::unauthorized(
            &locale,
            "token_invalid",
            Error::Other(format!("token: {} invalid", bearer.token())),
          ));
        }
      }
      Err(err) => {
        return Err(ServiceError::unauthorized(
          &locale,
          "token_invalid",
          Error::Other(format!("pasetors verify failed {:?}", err)),
        ));
      }
    }
  }
}
