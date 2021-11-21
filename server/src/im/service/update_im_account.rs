use crate::{
  account::model::{IdentityType, SignupData, SignupParam},
  alias::{KvPool, Pool},
  error::{Error, ServiceError},
  global::{Config, I18n, ImClient},
  im::{
    model::{
      ImCreateTokenParam, ImServerSigninParam, ImServerSignupResponse, ImServerSuccessResponse,
      ImServerTokenData, ImServerTokenInternalData, ImServerUpdateAccountParam, ImSignupParam,
      ImUpdateAccountParam,
    },
    service::{get_im_admin_token::get_or_create_admin_im_token, signup::signup},
  },
  middleware::Locale,
  types::ServiceResult,
  util::{id::next_id, string::get_random_letter},
};

use chrono::Utc;
use fluent_bundle::FluentArgs;
use serde_json::json;
use sqlx::query;

pub async fn update_im_account(kv: &KvPool, param: ImUpdateAccountParam) -> ServiceResult<()> {
  let cfg = Config::global();
  return Ok(());
  // let im_admin_token = get_or_create_admin_im_token(kv).await?;
  // let _: ImServerSignupResponse = ImClient::global()
  //   .post_with_token(
  //     "/user/update_user_info",
  //     &im_admin_token,
  //     json!(ImServerUpdateAccountParam {
  //       uid: param.account_id,
  //       name: param.name,
  //       icon: param.avatar,
  //       operationID: get_random_letter(32)
  //     })
  //     .to_string(),
  //   )
  //   .await?;

  // Ok(())
}
