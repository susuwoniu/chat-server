use super::token::Token;
use crate::config::Config;
use crate::util::key_pair::{Pair, RefreshTokenPair};
use crate::util::{datetime_tz, option_datetime_tz, option_string_i64, string_i64};
use chrono::prelude::{NaiveDate, NaiveDateTime};
use serde_json::Value;
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCodePostData {
    pub phone_country_code: i32,
    pub phone_number: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneAuthPostData {
    pub phone_country_code: i32,
    pub phone_number: String,
    pub code: String,
    pub timezone_in_seconds: i32,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthData {
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub access_token: String,
    #[serde(with = "datetime_tz")]
    pub expires_at: NaiveDateTime,
    pub refresh_token: String,
    #[serde(with = "datetime_tz")]
    pub refresh_token_expires_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneCodeMeta {
    pub length: usize,
    pub expires_in_minutes: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneCodeResponseData {
    pub meta: PhoneCodeMeta,
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
    #[serde(with = "option_datetime_tz")]
    pub avatar_updated_at: Option<NaiveDateTime>,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    pub approved: bool,
    #[serde(with = "option_datetime_tz")]
    pub approved_at: Option<NaiveDateTime>,
    #[serde(with = "option_string_i64")]
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
impl AuthData {
    pub fn new(
        account_id: i64,
        client_id: i64,
        pair: &Pair,
        config: &Config,
        refresh_token_pair: &RefreshTokenPair,
    ) -> Self {
        let token = Token::new(
            account_id,
            pair,
            config.auth.access_token_expires_in_minutes,
            config.server.url.clone().into(),
            config.server.url.clone().into(),
            client_id,
        );
        let access_token = token.get_token();

        let refresh = Token::new(
            account_id,
            &refresh_token_pair.0,
            config.auth.refresh_token_expires_in_days,
            config.server.url.clone().into(),
            config.server.url.clone().into(),
            client_id,
        );
        let refresh_token = refresh.get_token();
        // todo config
        let expires_at = token.get_expires_at();
        Self {
            account_id,
            access_token,
            expires_at: expires_at,
            refresh_token,
            refresh_token_expires_at: refresh.get_expires_at(),
        }
    }
}
