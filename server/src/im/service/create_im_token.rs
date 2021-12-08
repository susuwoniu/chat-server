use crate::{
  global::Config,
  im::model::{ImCreateTokenParam, ImServerTokenData},
  middleware::Locale,
  types::ServiceResult,
};
use chrono::Duration;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  aud: String, // Optional. Audience
  exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
  iat: usize, // Optional. Issued at (as UTC timestamp)
  iss: String, // Optional. Issuer
  nbf: usize, // Optional. Not Before (as UTC timestamp)
  sub: String,
}
pub async fn create_im_token(
  _: &Locale,
  param: ImCreateTokenParam,
) -> ServiceResult<ImServerTokenData> {
  let cfg = Config::global();
  let now = param.now;
  let iat = now;
  let nbf = iat;
  let expires_at = now + Duration::minutes(cfg.auth.access_token_expires_in_minutes);
  let im_username = format!("im{}", param.account_id);
  let im_claims = Claims {
    aud: cfg.im.domain.clone(),
    exp: expires_at.timestamp() as usize,
    iat: iat.timestamp() as usize,
    iss: cfg.server.url.to_string(),
    nbf: nbf.timestamp() as usize,
    sub: im_username,
  };
  let token = encode(
    &Header::new(Algorithm::RS256),
    &im_claims,
    &EncodingKey::from_rsa_pem(include_bytes!("../../../../config/im-jwt-private.pem"))?,
  )?;

  return Ok(ImServerTokenData {
    im_access_token: token,
    im_access_token_expires_at: expires_at.naive_utc(),
  });

  // let res: ServiceResult<ImServerSuccessResponse<ImServerTokenInternalData>> = ImClient::global()
  //   .post(
  //     "/auth/user_token",
  //     json!(ImServerSigninParam {
  //       secret: cfg.im.api_key.clone(),
  //       uid: param.account_id,
  //       platform: param.platform.clone().into(),
  //     })
  //     .to_string(),
  //   )
  //   .await;

  // dbg!(&res);
  // match res {
  //   Ok(res) => {
  //     return Ok(res.data.into());
  //   }
  //   Err(e) => {
  //     if param.try_signup && e.detail.contains("record not found") {
  //       // try register
  //       return signup(
  //         locale,
  //         ImSignupParam {
  //           account_id: param.account_id,
  //           try_login: false,
  //           platform: param.platform,
  //           name: param.name,
  //           avatar: param.avatar,
  //         },
  //       )
  //       .await;
  //     } else {
  //       return Err(e.into());
  //     }
  //   }
  // }
}
