use crate::{
  middleware::ClientPlatform,
  util::{datetime_tz, string_i64},
};
use chrono::{
  prelude::{DateTime, NaiveDateTime, Utc},
  serde::ts_seconds::deserialize as from_ts,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImSignupParam {
  pub account_id: i64,
  pub platform: i32,
  pub try_login: bool,
  pub name: String,
  pub avatar: Option<String>,
}

impl From<ClientPlatform> for i32 {
  fn from(platform: ClientPlatform) -> Self {
    match platform {
      ClientPlatform::IOS => 1,
      ClientPlatform::Android => 2,
      ClientPlatform::Web => 5,
      ClientPlatform::Windows => 3,
      ClientPlatform::MacOS => 4,
      ClientPlatform::Linux => 7,
      ClientPlatform::WechatMini => 6,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImCreateTokenParam {
  pub account_id: i64,
  pub platform: i32,
  pub try_signup: bool,
  pub avatar: Option<String>,
  pub name: String,
  pub now: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerSuccessResponse<T> {
  #[serde(rename = "errCode")]
  pub error_code: i64,
  #[serde(rename = "errMsg")]
  pub error_message: String,
  pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerTokenData {
  pub im_access_token: String,
  #[serde(with = "datetime_tz")]
  pub im_access_token_expires_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerTokenInternalData {
  #[serde(rename = "token")]
  pub im_access_token: String,
  #[serde(rename = "expiredTime")]
  #[serde(deserialize_with = "from_ts")]
  pub im_access_token_expires_at: DateTime<Utc>,
}
impl From<ImServerTokenInternalData> for ImServerTokenData {
  fn from(data: ImServerTokenInternalData) -> Self {
    ImServerTokenData {
      im_access_token: data.im_access_token,
      im_access_token_expires_at: data.im_access_token_expires_at.naive_utc(),
    }
  }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerSignupResponse {
  #[serde(rename = "errCode")]
  pub error_code: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerSignupParam {
  pub secret: String,
  pub platform: i32,
  #[serde(with = "string_i64")]
  pub uid: i64,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImUpdateAccountParam {
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub name: Option<String>,
  pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerUpdateAccountParam {
  #[serde(with = "string_i64")]
  pub uid: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<String>,
  pub operation_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerSigninParam {
  pub secret: String,
  pub platform: i32,
  #[serde(with = "string_i64")]
  pub uid: i64,
}
