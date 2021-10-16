use crate::util::id::next_id;
use crate::util::string::get_random_letter;
use chrono::prelude::{NaiveDate, NaiveDateTime};
use serde_json::Value;
use shrinkwraprs::Shrinkwrap;

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

#[derive(Debug, Deserialize)]
pub struct PhoneLoginData {
    pub phone_country_code: i32,
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimAccount {
    pub id: i64,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedAccount(pub Option<SlimAccount>);

impl From<SlimAccount> for LoggedAccount {
    fn from(slim_account: SlimAccount) -> Self {
        LoggedAccount(Some(slim_account))
    }
}

impl From<PhoneLoginData> for InsertableAccount {
    fn from(account_data: PhoneLoginData) -> Self {
        let PhoneLoginData {
            phone_country_code,
            phone_number,
        } = account_data;
        let id = next_id();
        // get random name
        // let default_name = i18n.get_by_lang("default-name", "zh-Hans");
        let default_name = get_random_letter(4);
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
