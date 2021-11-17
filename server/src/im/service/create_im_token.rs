use crate::{
  account::model::{IdentityType, SignupData, SignupParam},
  alias::Pool,
  error::{Error, ServiceError},
  global::{Config, I18n, ImClient},
  im::{
    model::{
      ImCreateTokenParam, ImServerSigninParam, ImServerSignupResponse, ImServerSuccessResponse,
      ImServerTokenData, ImServerTokenInternalData, ImSignupParam,
    },
    service::signup::signup,
  },
  middleware::Locale,
  types::ServiceResult,
  util::{id::next_id, string::get_random_letter},
};
use chrono::Utc;
use fluent_bundle::FluentArgs;
use serde_json::json;
use sqlx::query;
pub async fn create_im_token(
  locale: &Locale,
  param: ImCreateTokenParam,
) -> ServiceResult<ImServerTokenData> {
  let cfg = Config::global();

  let res: ServiceResult<ImServerSuccessResponse<ImServerTokenInternalData>> = ImClient::global()
    .post(
      "/auth/user_token",
      json!(ImServerSigninParam {
        secret: cfg.im.api_key.clone(),
        uid: param.account_id,
        platform: param.platform.clone().into(),
      })
      .to_string(),
    )
    .await;

  dbg!(&res);
  match res {
    Ok(res) => {
      return Ok(res.data.into());
    }
    Err(e) => {
      if param.try_signup && e.detail.contains("record not found") {
        // try register
        return signup(
          locale,
          ImSignupParam {
            account_id: param.account_id,
            try_login: false,
            platform: param.platform,
            name: param.name,
            avatar: param.avatar,
          },
        )
        .await;
      } else {
        return Err(e.into());
      }
    }
  }
}
