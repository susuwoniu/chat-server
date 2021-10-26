use crate::util::{datetime_tz, option_datetime_tz, option_string_i64, string_i64};
use chrono::prelude::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SigninType {
  PhoneCode,
  SignupWithPhoneCode,
  RefreshToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninParam {
  pub account_id: i64,
  pub account_auth_id: i64,
  pub client_id: i64,
  pub device_id: String,
  pub signin_type: SigninType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninWithPhoneParam {
  pub phone_country_code: i32,
  pub phone_number: String,
  pub code: String,
  pub timezone_in_seconds: i32,
  pub client_id: i64,
  pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
  Unknown,
  Male,
  Female,
  Other,
  Intersex,
}
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "identity_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum IdentityType {
  Phone,
  Email,
  Wechat,
  Weibo,
  Apple,
  Google,
  Facebook,
  Twitter,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginActivityData {
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub account_auth_id: i64,
  pub last_signin_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimAccount {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub name: String,
  pub bio: String,
  pub gender: Gender,
  pub vip: bool,
  pub posts_count: i64,
  pub likes_count: i64,
  pub show_age: bool,
  pub show_distance: bool,
  pub suspended: bool,
  #[serde(with = "option_datetime_tz")]
  pub suspended_at: Option<NaiveDateTime>,
  #[serde(with = "option_datetime_tz")]
  pub suspended_until: Option<NaiveDateTime>,
  pub suspended_reason: Option<String>,
  pub location: Option<String>,
  pub avatar: Option<String>,
  pub age: Option<i32>,
  pub profile_images: Option<Value>,
  #[serde(with = "option_datetime_tz")]
  pub avatar_updated_at: Option<NaiveDateTime>,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  pub approved: bool,
  #[serde(with = "option_datetime_tz")]
  pub approved_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct GetAccountPathParam {
  pub account_id: i64,
}
#[derive(Debug, Deserialize)]
pub struct SendPhoneCodePathParam {
  pub phone_country_code: i32,
  pub phone_number: String,
}
#[derive(Debug, Deserialize)]
pub struct PhoneAuthPathParam {
  pub phone_country_code: i32,
  pub phone_number: String,
  pub code: String,
}
#[derive(Debug, Deserialize)]
pub struct PhoneAuthBodyParam {
  pub timezone_in_seconds: i32,
  pub device_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneCodeMeta {
  pub length: usize,
  pub expires_in_minutes: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceParam {
  pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneCodeResponseData {
  pub meta: PhoneCodeMeta,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuccessMeta {
  pub ok: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuccessResponseData {
  pub meta: SuccessMeta,
}
impl Default for SuccessResponseData {
  fn default() -> Self {
    Self {
      meta: SuccessMeta { ok: true },
    }
  }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub name: String,
  pub bio: String,
  pub gender: Gender,
  pub admin: bool,
  pub moderator: bool,
  pub vip: bool,
  pub posts_count: i64,
  pub likes_count: i64,
  pub show_age: bool,
  pub show_distance: bool,
  pub age: Option<i32>,
  pub suspended: bool,
  #[serde(with = "option_datetime_tz")]
  pub suspended_at: Option<NaiveDateTime>,
  #[serde(with = "option_datetime_tz")]
  pub suspended_until: Option<NaiveDateTime>,
  pub suspended_reason: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub timezone_in_seconds: Option<i32>,
  pub phone_country_code: Option<i32>,
  pub phone_number: Option<String>,
  pub location: Option<String>,
  pub country_id: Option<i32>,
  pub state_id: Option<i32>,
  pub city_id: Option<i32>,
  pub avatar: Option<String>,
  pub profile_images: Option<Value>,
  #[serde(default)]
  #[serde(with = "option_datetime_tz")]
  pub avatar_updated_at: Option<NaiveDateTime>,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  pub approved: bool,
  #[serde(default)]
  #[serde(with = "option_datetime_tz")]
  pub approved_at: Option<NaiveDateTime>,
  #[serde(default)]
  #[serde(with = "option_string_i64")]
  pub invite_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountParam {
  pub name: Option<String>,
  pub bio: Option<String>,
  pub gender: Option<Gender>,
  pub admin: Option<bool>,
  pub moderator: Option<bool>,
  pub vip: Option<bool>,
  pub show_age: Option<bool>,
  pub show_distance: Option<bool>,
  pub suspended: Option<bool>,
  #[serde(default)]
  #[serde(with = "option_datetime_tz")]
  pub suspended_at: Option<NaiveDateTime>,
  #[serde(default)]
  #[serde(with = "option_datetime_tz")]
  pub suspended_until: Option<NaiveDateTime>,
  pub suspended_reason: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub timezone_in_seconds: Option<i32>,
  pub phone_country_code: Option<i32>,
  pub phone_number: Option<String>,
  pub location: Option<String>,
  pub country_id: Option<i32>,
  pub state_id: Option<i32>,
  pub city_id: Option<i32>,
  pub avatar: Option<String>,
  pub profile_images: Option<Value>,
  pub approved: Option<bool>,
  pub invite_id: Option<i64>,
}

impl From<Account> for SlimAccount {
  fn from(account: Account) -> Self {
    let Account {
      id,
      name,
      bio,
      gender,
      vip,
      posts_count,
      likes_count,
      show_age,
      show_distance,
      suspended,
      suspended_at,
      suspended_until,
      suspended_reason,
      location,
      avatar,
      age,
      profile_images,
      avatar_updated_at,
      created_at,
      updated_at,
      approved,
      approved_at,
      ..
    } = account;

    Self {
      id,
      name,
      bio,
      gender,
      vip,
      posts_count,
      likes_count,
      show_age,
      show_distance,
      suspended,
      suspended_at,
      suspended_until,
      suspended_reason,
      location,
      avatar,
      age,
      profile_images,
      avatar_updated_at,
      created_at,
      updated_at,
      approved,
      approved_at,
    }
  }
}
