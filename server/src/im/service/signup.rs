use crate::{
  account::model::{IdentityType, SignupData, SignupParam},
  alias::Pool,
  error::{Error, ServiceError},
  global::{Config, I18n, ImClient},
  im::{
    model::{
      ImCreateTokenParam, ImServerSignupParam, ImServerSignupResponse, ImServerSuccessResponse,
      ImServerTokenData, ImServerTokenInternalData, ImSignupParam,
    },
    service::create_im_token::create_im_token,
  },
  middleware::Locale,
  types::ServiceResult,
  util::{id::next_id, string::get_random_letter},
};

use chrono::Utc;
use fluent_bundle::FluentArgs;
use serde_json::json;
use sqlx::query;
pub async fn signup(locale: &Locale, param: ImSignupParam) -> ServiceResult<ImServerTokenData> {
  let cfg = Config::global();

  let res: ImServerSuccessResponse<ImServerTokenInternalData> = ImClient::global()
    .post(
      "/auth/user_register",
      json!(ImServerSignupParam {
        secret: cfg.im.api_key.clone(),
        platform: param.platform,
        uid: param.account_id,
        name: param.name.clone(),
        icon: param.avatar
      })
      .to_string(),
    )
    .await?;
  Ok(res.data.into())
}