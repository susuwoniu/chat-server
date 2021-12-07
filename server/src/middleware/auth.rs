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
#[derive(Debug, Clone)]
pub struct Auth {
  pub account_id: i64,
  pub client_id: i64,
  pub token_id: i64,
  pub device_id: String,
  pub admin: bool,
  pub moderator: bool,
  pub vip: bool,
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
        let sub_option = result.get_claim("sub");
        let client_id_option = result.get_claim("client_id");
        let roles_option = result.get_claim("roles");
        let jti_option = result.get_claim("jti");
        let device_id_option = result.get_claim("device_id");
        if let (Some(sub), Some(client_id), Some(roles), Some(jti), Some(device_id)) = (
          sub_option,
          client_id_option,
          roles_option,
          jti_option,
          device_id_option,
        ) {
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

                // check roles
                let roles = roles.as_str();
                if let Some(roles) = roles {
                  let admin = roles.contains("admin");
                  let moderator = roles.contains("moderator");
                  let vip = roles.contains("vip");

                  // get token id
                  let jti = jti.as_str();

                  if let Some(jti) = jti {
                    let jti = i64::from_str(jti).map_err(|_| {
                      ServiceError::unauthorized(
                        &locale,
                        "token_jti_invalid",
                        Error::Other(format!("token: {} , jti: {:?}", bearer.token(), jti)),
                      )
                    })?;

                    let device_id = device_id.as_str();

                    if let Some(device_id) = device_id {
                      // test TODO
                      // return Err(ServiceError::unauthorized(
                      //   &locale,
                      //   "token_invalid",
                      //   Error::Other(format!("pasetors verify failed {:?}", Error::Default)),
                      // ));
                      // verified
                      return Ok(Self {
                        account_id: sub,
                        client_id,
                        admin,
                        moderator,
                        device_id: device_id.to_string(),
                        vip,
                        token_id: jti,
                      });
                    } else {
                      return Err(ServiceError::unauthorized(
                        &locale,
                        "token_device_id_invalid",
                        Error::Other(format!(
                          "token: {} , device_id: {:?}",
                          bearer.token(),
                          device_id
                        )),
                      ));
                    }
                  } else {
                    return Err(ServiceError::unauthorized(
                      &locale,
                      "token_jti_invalid",
                      Error::Other(format!("token: {} , jti: {:?}", bearer.token(), jti)),
                    ));
                  }
                } else {
                  return Err(ServiceError::unauthorized(
                    &locale,
                    "token_roles_invalid",
                    Error::Other(format!("token: {} , roles: {:?}", bearer.token(), roles)),
                  ));
                }
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
