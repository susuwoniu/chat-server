use super::token::Token;
use crate::config::Config;
use crate::util::id::next_id;
use crate::util::key_pair::Pair;
use chrono::prelude::{NaiveDate, NaiveDateTime, Utc};
use chrono::Duration;
use serde_json::Value;
use shrinkwraprs::Shrinkwrap;

#[derive(Debug, Deserialize)]
pub struct PhoneCodePostData {
    pub phone_country_code: i32,
    pub phone_number: String,
}
#[derive(Debug, Deserialize)]
pub struct PhoneAuthPostData {
    pub phone_country_code: i32,
    pub phone_number: String,
    pub code: String,
}
#[derive(Debug, Deserialize, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
pub enum Gender {
    Unknown,
    Male,
    Female,
    Other,
    Intersex,
}
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "identity_type", rename_all = "snake_case")]
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
#[derive(Debug, Deserialize)]
pub struct LoginActivityData {
    pub account_id: i64,
    pub account_auth_id: i64,
    pub last_signin_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimAccount {
    pub id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthData {
    pub account_id: i64,
    pub access_token: String,
    pub expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: NaiveDateTime,
}

impl AuthData {
    pub fn new(account_id: i64, client_id: i64, pair: &Pair, config: &Config) -> Self {
        let token = Token::new(
            account_id,
            pair,
            config.auth.access_token_expires_in_days,
            config.server.url.clone().into(),
            config.server.url.clone().into(),
            client_id,
        );
        let access_token = token.get_token();
        let refresh_token = "".to_string();
        let now = Utc::now();
        // todo config
        let expires_at = token.get_expires_at();
        let refresh_token_expires_at = now + Duration::days(365);
        Self {
            account_id,
            access_token,
            expires_at: expires_at,
            refresh_token,
            refresh_token_expires_at: refresh_token_expires_at.naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub bio: String,
    pub posts_count: i64,
    pub likes_count: i64,
    pub show_age: bool,
    pub show_distance: bool,
    pub suspended: bool,
    pub deleted: bool,
    pub suspended_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDate>,
    pub phone_country_code: Option<i32>,
    pub phone_number: Option<String>,
    pub location: Option<String>,
    pub country_id: Option<i32>,
    pub state_id: Option<i32>,
    pub city_id: Option<i32>,
    pub avatar: Option<String>,
    pub profile_images: Option<Value>,
    pub avatar_updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAuth {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub sign_in_count: i32,
    pub name: String,
    pub bio: String,
    pub likes_count: i64,
    pub show_age: bool,
    pub show_distance: bool,
    pub suspended: bool,
    pub deleted: bool,
    pub suspended_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDate>,
    pub phone_country_code: Option<i32>,
    pub phone_number: Option<String>,
    pub location: Option<String>,
    pub country_id: Option<i32>,
    pub state_id: Option<i32>,
    pub city_id: Option<i32>,
    pub avatar: Option<String>,
    pub profile_images: Option<Value>,
    pub avatar_updated_at: Option<NaiveDateTime>,
}
#[derive(Debug)]
pub struct InsertableAccount {
    pub id: i64,
    pub name: String,
    pub bio: String,
    pub phone_country_code: i32,
    pub phone_number: String,
    pub birthday: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub posts_count: i64,
    pub likes_count: i64,
    pub show_age: bool,
    pub show_distance: bool,
    pub suspended: bool,
    pub deleted: bool,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedAccount(pub Option<SlimAccount>);

impl From<SlimAccount> for LoggedAccount {
    fn from(slim_account: SlimAccount) -> Self {
        LoggedAccount(Some(slim_account))
    }
}

impl From<PhoneCodePostData> for InsertableAccount {
    fn from(account_data: PhoneCodePostData) -> Self {
        let PhoneCodePostData {
            phone_country_code,
            phone_number,
        } = account_data;
        Self {
            id: next_id(),
            phone_country_code,
            phone_number,
            bio: "".to_string(),
            birthday: chrono::Local::now().date().naive_local(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            name: "".to_string(),
            posts_count: 0,
            likes_count: 0,
            show_age: true,
            show_distance: true,
            suspended: false,
            deleted: false,
        }
    }
}
impl From<Account> for SlimAccount {
    fn from(account: Account) -> Self {
        let Account { id, .. } = account;

        Self { id }
    }
}
