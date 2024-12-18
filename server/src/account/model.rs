use crate::{
    error::ServiceError,
    middleware::ClientPlatform,
    types::{
        Action, AvatarVersion, FieldAction, FieldUpdateAction, Gender, Image, PushServiceType,
    },
    util::{
        base62_i64, base62_to_i64, datetime_tz, option_datetime_tz, option_string_i64, string_i64,
    },
};
use chrono::prelude::{DateTime, NaiveDate, NaiveDateTime, Utc};
use ipnetwork17::IpNetwork;
use jsonapi::{api::*, jsonapi_model, model::*};
use queue_file::QueueFile;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    pub device_token: Option<String>,
    pub push_service_type: Option<PushServiceType>,
    pub signin_type: SigninType,
    pub ip: IpNetwork,
    pub client_platform: ClientPlatform,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPutDevicePathParam {
    pub device_token: String,
    pub push_service_type: PushServiceType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignouttParam {
    pub device_token: Option<String>,
    pub push_service_type: Option<PushServiceType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PutDeviceParam {
    pub device_token: String,
    pub client_platform: ClientPlatform,
    pub push_service_type: PushServiceType,
    pub account_id: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceData {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub device_token: String,
    pub client_platform: ClientPlatform,
    pub push_service_type: PushServiceType,
    #[serde(default)]
    #[serde(with = "option_string_i64")]
    pub account_id: Option<i64>,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
}
jsonapi_model!(DeviceData; "devices");

#[derive(Debug)]
pub struct SigninWithPhoneParam {
    pub phone_country_code: i32,
    pub phone_number: String,
    pub code: String,
    pub timezone_in_seconds: i32,
    pub client_id: i64,
    pub device_id: String,
    pub ip: IpNetwork,
    pub client_platform: ClientPlatform,
    pub qf_mutex: Arc<Mutex<QueueFile>>,
    pub device_token: Option<String>,
    pub push_service_type: Option<PushServiceType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupParam {
    pub identity_type: IdentityType,
    pub identifier: String,
    pub phone_country_code: Option<i32>,
    pub phone_number: Option<String>,
    pub timezone_in_seconds: i32,
    pub ip: IpNetwork,
    pub client_platform: ClientPlatform,
    pub admin: bool,
    pub device_token: Option<String>,
    pub push_service_type: Option<PushServiceType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupData {
    pub account_id: i64,
    pub account_auth_id: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[repr(i16)]
#[serde(rename_all = "snake_case")]
pub enum IdentityType {
    Phone = 1,
    Email = 2,
    Wechat = 3,
    Weibo = 4,
    Apple = 5,
    Google = 6,
    Facebook = 7,
    Twitter = 8,
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
    pub account: FullAccount,
    pub im_username: String,
    pub im_access_token: String,
    #[serde(with = "datetime_tz")]
    pub im_access_token_expires_at: NaiveDateTime,
}
jsonapi_model!(AuthData; "tokens"; has one account);

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginActivityData {
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub account_auth_id: i64,
    pub last_signin_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub name: String,
    pub bio: String,
    pub gender: Gender,
    pub vip: bool,
    pub post_count: i64,
    pub like_count: i64,
    pub favorite_count: i64,
    pub show_age: bool,
    pub show_distance: bool,
    pub show_viewed_action: bool,
    pub suspended: bool,
    #[serde(with = "option_datetime_tz")]
    pub suspended_at: Option<NaiveDateTime>,
    #[serde(with = "option_datetime_tz")]
    pub suspended_until: Option<NaiveDateTime>,
    pub suspended_reason: Option<String>,
    pub location: Option<String>,
    pub avatar: Option<Image>,
    pub age: Option<i32>,
    #[serde(with = "option_datetime_tz")]
    pub avatar_updated_at: Option<NaiveDateTime>,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    pub approved: bool,
    #[serde(with = "option_datetime_tz")]
    pub approved_at: Option<NaiveDateTime>,
    pub profile_images: Vec<Image>,
    pub is_liked: Option<bool>,
    pub is_blocked: Option<bool>,
    #[serde(with = "option_datetime_tz")]
    pub last_post_created_at: Option<NaiveDateTime>,
    #[serde(with = "datetime_tz")]
    pub next_post_not_before: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenParam {
    pub device_id: String,
    pub device_token: Option<String>,
    pub push_service_type: Option<PushServiceType>,
}

jsonapi_model!(Account; "accounts");

#[derive(Debug, Deserialize)]
pub struct GetAccountPathParam {
    pub account_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GetAccountsParam {
    pub ids: Vec<i64>,
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
    pub device_token: Option<String>,
    pub push_service_type: Option<PushServiceType>,
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
pub struct PutImageParam {
    pub url: String,
    pub width: f64,
    pub height: f64,
    pub size: i64,
    pub mime_type: String,
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
pub struct FullAccount {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub name: String,
    pub bio: String,
    pub gender: Gender,
    pub admin: bool,
    pub moderator: bool,
    pub vip: bool,
    pub post_count: i64,
    pub like_count: i64,
    pub favorite_count: i64,
    pub show_age: bool,
    pub show_distance: bool,
    pub show_viewed_action: bool,
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
    pub avatar: Option<Image>,
    pub profile_images: Vec<Image>,
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
    pub post_template_count: i64,
    #[serde(with = "option_datetime_tz")]
    pub last_post_created_at: Option<NaiveDateTime>,
    #[serde(with = "datetime_tz")]
    pub next_post_not_before: NaiveDateTime,
    pub is_can_post: bool,
    pub now: DateTime<Utc>,
    pub next_post_in_seconds: i64,
    pub agree_community_rules_at: Option<NaiveDateTime>,
    pub bio_updated_at: Option<NaiveDateTime>,
    pub name_updated_at: Option<NaiveDateTime>,
}
jsonapi_model!(FullAccount; "full-accounts");
#[derive(Debug, Clone)]

pub struct DbAccountView {
    pub id: i64,
    pub viewed_count: i64,
    pub viewed_by: i64,
    pub target_account_id: i64,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub time_cursor: i64,
}

#[derive(Debug, Clone)]

pub struct DbAccountLike {
    pub id: i64,
    pub account_id: i64,
    pub target_account_id: i64,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]

pub struct DbAccountBlock {
    pub id: i64,
    pub account_id: i64,
    pub target_account_id: i64,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLiked {
    #[serde(with = "string_i64")]
    pub id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "string_i64")]
    pub target_account_id: i64,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub account: Account,
    #[serde(with = "base62_i64")]
    pub cursor: i64,
}
jsonapi_model!(AccountLiked; "account-liked"; has one account);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLike {
    #[serde(with = "string_i64")]
    pub id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "string_i64")]
    pub target_account_id: i64,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub target_account: Account,
    #[serde(with = "base62_i64")]
    pub cursor: i64,
}
jsonapi_model!(AccountLike; "account-likes"; has one target_account);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBlock {
    #[serde(with = "string_i64")]
    pub id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "string_i64")]
    pub target_account_id: i64,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub target_account: Account,
    #[serde(with = "base62_i64")]
    pub cursor: i64,
}
jsonapi_model!(AccountBlock; "account-blocks"; has one target_account);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountView {
    #[serde(with = "string_i64")]
    pub id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "string_i64")]
    pub target_account_id: i64,
    #[serde(with = "string_i64")]
    pub viewed_by: i64,
    pub viewed_by_account: Account,
    #[serde(with = "base62_i64")]
    pub cursor: i64,
    pub viewed_count: i64,
}
jsonapi_model!(AccountView; "account-views"; has one viewed_by_account);
pub struct DbAccount {
    pub id: i64,
    pub name: String,
    pub bio: String,
    pub gender: Gender,
    pub admin: bool,
    pub moderator: bool,
    pub vip: bool,
    pub post_count: i64,
    pub like_count: i64,
    pub favorite_count: i64,
    pub show_age: bool,
    pub show_viewed_action: bool,
    pub show_distance: bool,
    pub suspended: bool,
    pub suspended_at: Option<NaiveDateTime>,
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
    pub avatar: Option<Value>,
    pub avatar_updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub approved: bool,
    pub approved_at: Option<NaiveDateTime>,
    pub invite_id: Option<i64>,
    pub name_change_count: i32,
    pub bio_change_count: i32,
    pub birthday_change_count: i32,
    pub phone_change_count: i32,
    pub gender_change_count: i32,
    pub post_template_count: i64,
    pub profile_image_change_count: i32,
    pub profile_images: Option<Value>,
    pub last_post_created_at: Option<NaiveDateTime>,
    pub agree_community_rules_at: Option<NaiveDateTime>,
    pub gender_updated_at: Option<NaiveDateTime>,
    pub bio_updated_at: Option<NaiveDateTime>,
    pub name_updated_at: Option<NaiveDateTime>,
    pub avatar_change_count: i32,
    pub birthday_updated_at: Option<NaiveDateTime>,
    pub phone_updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiAccountViewFilter {
    pub after: Option<String>,
    pub before: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub limit: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountViewFilter {
    pub after: Option<i64>,
    pub before: Option<i64>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub limit: Option<i64>,
}
impl TryFrom<ApiAccountViewFilter> for AccountViewFilter {
    type Error = ServiceError;

    fn try_from(value: ApiAccountViewFilter) -> Result<Self, Self::Error> {
        let mut after = None;
        if let Some(after_value) = value.after {
            after = Some(base62_to_i64(&after_value)?);
        }
        let mut before = None;
        if let Some(before_value) = value.before {
            before = Some(base62_to_i64(&before_value)?);
        }
        Ok(Self {
            limit: value.limit,
            after,
            before,
            start_time: value.start_time,
            end_time: value.end_time,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiAccountLikeFilter {
    pub after: Option<String>,
    pub before: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub limit: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountLikeFilter {
    pub after: Option<i64>,
    pub before: Option<i64>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub limit: Option<i64>,
}
impl TryFrom<ApiAccountLikeFilter> for AccountLikeFilter {
    type Error = ServiceError;

    fn try_from(value: ApiAccountLikeFilter) -> Result<Self, Self::Error> {
        let mut after = None;
        if let Some(after_value) = value.after {
            after = Some(base62_to_i64(&after_value)?);
        }
        let mut before = None;
        if let Some(before_value) = value.before {
            before = Some(base62_to_i64(&before_value)?);
        }
        Ok(Self {
            limit: value.limit,
            after,
            before,
            start_time: value.start_time,
            end_time: value.end_time,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbProfileImage {
    #[serde(with = "string_i64")]
    pub id: i64,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub url: String,
    pub width: f64,
    pub height: f64,
    pub size: i64,
    pub mime_type: String,
    #[serde(rename = "order")]
    pub _order: i16,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbAvatarJson {
    pub version: AvatarVersion,
    pub image: Image,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ApiUpdateOtherAccountParam {
    pub viewed_count_action: Option<FieldAction>,
    pub like_count_action: Option<FieldAction>,
    pub block_count_action: Option<FieldAction>,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UpdateOtherAccountParam {
    pub viewed_count_action: Option<FieldAction>,
    pub like_count_action: Option<FieldAction>,
    pub block_count_action: Option<FieldAction>,
    pub target_account_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UpdateAccountParam {
    pub account_id: Option<i64>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub gender: Option<Gender>,
    pub admin: Option<bool>,
    pub moderator: Option<bool>,
    pub vip: Option<bool>,
    pub show_age: Option<bool>,
    pub avatar: Option<Image>,
    pub show_distance: Option<bool>,
    pub show_viewed_action: Option<bool>,
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
    pub bio_action: Option<FieldUpdateAction>,
    pub avatar_action: Option<FieldUpdateAction>,
    pub post_template_count_action: Option<FieldAction>,
    pub post_count_action: Option<FieldAction>,
    pub like_count_action: Option<FieldAction>,
    pub favorite_count_action: Option<FieldAction>,
    pub last_post_created_at: Option<NaiveDateTime>,
    pub agree_community_rules: Option<bool>,
    pub profile_images: Option<Vec<Image>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountImagesParam {
    pub images: Vec<UpdateAccountImageParam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountImageParam {
    pub order: i16,
    pub url: String,
    pub width: f64,
    pub height: f64,
    pub size: i64,
    pub mime_type: String,
}
impl From<FullAccount> for Account {
    fn from(account: FullAccount) -> Self {
        let FullAccount {
            id,
            name,
            bio,
            gender,
            vip,
            post_count,
            like_count,
            show_age,
            show_distance,
            suspended,
            suspended_at,
            suspended_until,
            suspended_reason,
            location,
            avatar,
            age,
            avatar_updated_at,
            created_at,
            updated_at,
            approved,
            approved_at,
            show_viewed_action,
            profile_images,
            last_post_created_at,
            next_post_not_before,
            favorite_count,
            ..
        } = account;

        Self {
            id,
            name,
            bio,
            gender,
            vip,
            post_count,
            like_count,
            show_age,
            show_distance,
            suspended,
            suspended_at,
            suspended_until,
            suspended_reason,
            location,
            avatar,
            age,
            avatar_updated_at,
            created_at,
            updated_at,
            approved,
            approved_at,
            show_viewed_action,
            profile_images,
            is_liked: None,
            is_blocked: None,
            last_post_created_at,
            next_post_not_before,
            favorite_count,
        }
    }
}
