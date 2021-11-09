use crate::{
  types::Action,
  util::{datetime_tz, option_datetime_tz, option_string_i64, string_i64},
};
use chrono::prelude::{NaiveDate, NaiveDateTime};
use ipnetwork17::IpNetwork;
use jsonapi::{api::*, array::JsonApiArray, jsonapi_model, model::*};
use serde::{Deserialize, Serialize};

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
  pub ip: IpNetwork,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninWithPhoneParam {
  pub phone_country_code: i32,
  pub phone_number: String,
  pub code: String,
  pub timezone_in_seconds: i32,
  pub client_id: i64,
  pub device_id: String,
  pub ip: IpNetwork,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupParam {
  pub identity_type: IdentityType,
  pub identifier: String,
  pub phone_country_code: Option<i32>,
  pub phone_number: Option<String>,
  pub timezone_in_seconds: i32,
  pub ip: IpNetwork,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupData {
  pub account_id: i64,
  pub account_auth_id: i64,
}
#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
  Unknown,
  Male,
  Female,
  Other,
  Intersex,
}
#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenType {
  Bearer,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthData {
  #[serde(with = "string_i64")]
  pub id: i64,
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub device_id: String,
  pub access_token: String,
  pub access_token_type: TokenType,
  #[serde(with = "datetime_tz")]
  pub access_token_expires_at: NaiveDateTime,
  pub refresh_token: String,
  #[serde(with = "string_i64")]
  pub refresh_token_id: i64,
  pub refresh_token_type: TokenType,
  #[serde(with = "datetime_tz")]
  pub refresh_token_expires_at: NaiveDateTime,
  pub actions: Vec<Action>,
  pub account: Account,
}
jsonapi_model!(AuthData; "tokens"; has one account);

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
  pub profile_images: Vec<ProfileImage>,
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

jsonapi_model!(SlimAccount; "slim-accounts"; has many profile_images);

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
pub struct AddImageParam {
  pub url: String,
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
  pub profile_images: Vec<ProfileImage>,
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
  pub name_change_count: i32,
  pub bio_change_count: i32,
  pub birthday_change_count: i32,
  pub phone_change_count: i32,
  pub gender_change_count: i32,
  pub actions: Vec<Action>,
}
jsonapi_model!(Account; "accounts"; has many profile_images);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileImage {
  #[serde(with = "string_i64")]
  pub id: i64,
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub url: String,
  #[serde(rename = "order")]
  pub sequence: i32,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
}
jsonapi_model!(ProfileImage; "profile-images");
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
  pub approved: Option<bool>,
  pub invite_id: Option<i64>,
  pub skip_optional_info: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountImageParam {
  #[serde(rename = "order")]
  pub sequence: i32,
  pub url: String,
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
