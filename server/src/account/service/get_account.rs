use crate::{
    account::model::{
        Account, AccountView, AccountViewFilter, DbAccount, DbAccountView, DbProfileImagesJson,
        FullAccount, ProfileImage,
    },
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::Locale,
    types::{Action, ActionType, DataWithPageInfo, Gender, JsonVersion, PageInfo, ServiceResult},
};
use chrono::offset::FixedOffset;
use chrono::Datelike;
use chrono::{Date, Utc};
use itertools::Itertools;
use sqlx::query_as;
use std::iter::Iterator;
pub async fn get_db_account(
    locale: &Locale,
    pool: &Pool,
    account_id: i64,
) -> ServiceResult<DbAccount> {
    let row=  query_as!(DbAccount,
    r#"
      select id,name,bio,gender as "gender:Gender",admin,moderator,vip,post_count,like_count,show_age,show_distance,show_viewed_action,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id,name_change_count,bio_change_count,gender_change_count,birthday_change_count,phone_change_count,skip_optional_info,profile_image_change_count,post_template_count,profile_images from accounts where id = $1 and deleted=false
"#,
account_id
  )
  .fetch_optional(pool)
  .await?;
    if let Some(row) = row {
        return Ok(row);
    } else {
        return Err(ServiceError::account_not_exist(
            locale,
            Error::Other(format!("Can not found account_id: {} at db", account_id)),
        ));
    }
}
pub async fn get_db_account_views(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountViewFilter,
    target_account_id: i64,
) -> ServiceResult<Vec<DbAccountView>> {
    let cfg = Config::global();
    let mut limit = cfg.page_size;
    if let Some(filter_limit) = filter.limit {
        if filter_limit > cfg.max_page_size {
            return Err(ServiceError::bad_request(
                locale,
                "limit_is_too_large",
                Error::Other(format!(
                    "limit {} is too large to max limit {}",
                    filter_limit, cfg.max_page_size
                )),
            ));
        } else {
            limit = filter_limit;
        }
    }
    let rows=  query_as!(DbAccountView,
    r#"
      select id,viewed_by,viewed_count,updated_at,created_at,target_account_id from account_views where 
      target_account_id = $1 
      and ($2::bigint is null or id > $2) 
      and ($3::bigint is null or id < $3) 
      and ($4::timestamp is null or created_at > $4)
      and ($5::timestamp is null or created_at < $5)
      order by id desc 
      limit $6
"#,
target_account_id,
filter.before,
filter.after,
filter.start_time,
filter.end_time,
limit
  )
  .fetch_all(pool)
  .await?;
    return Ok(rows);
}
pub async fn get_account_views(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountViewFilter,
    target_account_id: i64,
) -> ServiceResult<DataWithPageInfo<AccountView>> {
    let rows = get_db_account_views(locale, pool, filter, target_account_id).await?;
    // get accounts info
    let accounts = get_accounts(
        locale,
        pool,
        rows.clone().into_iter().map(|row| row.viewed_by).collect(),
    )
    .await?;

    let account_map = accounts
        .into_iter()
        .map(|account| (account.id, account))
        .collect::<std::collections::HashMap<_, _>>();

    let data: Vec<AccountView> = rows
        .into_iter()
        .filter_map(|row| {
            let account = account_map.get(&row.viewed_by);
            if let Some(account) = account {
                return Some(format_account_view(row, account.clone()));
            } else {
                return None;
            }
        })
        .collect();
    let mut start = None;
    let mut end = None;
    if let Some(row) = data.first() {
        start = Some(row.cursor);
    }
    if let Some(row) = data.last() {
        end = Some(row.cursor);
    }
    let collection = DataWithPageInfo::<AccountView> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(collection);
}
#[allow(dead_code)]
pub async fn get_db_account_view(
    _: &Locale,
    pool: &Pool,
    target_account_id: i64,
    viewed_by: i64,
) -> ServiceResult<Option<DbAccountView>> {
    let row=  query_as!(DbAccountView,
    r#"
      select id,viewed_by,viewed_count,updated_at,created_at,target_account_id from account_views where target_account_id = $1 and viewed_by = $2
"#,
target_account_id,
viewed_by
  )
  .fetch_optional(pool)
  .await?;
    return Ok(row);
}
async fn get_db_accounts(
    locale: &Locale,
    pool: &Pool,
    account_ids: Vec<i64>,
) -> ServiceResult<Vec<DbAccount>> {
    let cfg = Config::global();
    if account_ids.len() > cfg.max_accounts {
        return Err(ServiceError::bad_request(
            locale,
            "reach_max_accounts_limit",
            Error::Default,
        ));
    }
    let rows = query_as!(DbAccount,
    r#"
      select id,name,bio,gender as "gender:Gender",admin,moderator,vip,post_count,like_count,show_age,show_distance,show_viewed_action,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id,name_change_count,bio_change_count,gender_change_count,birthday_change_count,phone_change_count,skip_optional_info,profile_image_change_count,post_template_count,profile_images from accounts where id = ANY ($1::bigint[]) and deleted=false
"#,
&account_ids
  )
  .fetch_all(pool)
  .await?;
    return Ok(rows);
}
pub async fn get_accounts(
    locale: &Locale,
    pool: &Pool,
    account_ids: Vec<i64>,
) -> ServiceResult<Vec<Account>> {
    let db_accounts =
        get_db_accounts(locale, pool, account_ids.into_iter().unique().collect()).await?;
    return Ok(db_accounts
        .into_iter()
        .map(|db_account: DbAccount| {
            return format_account(db_account).into();
        })
        .collect());
}
pub async fn get_account(locale: &Locale, pool: &Pool, account_id: i64) -> ServiceResult<Account> {
    let db_account = get_db_account(locale, pool, account_id).await?;
    return Ok(Account::from(format_account(db_account)));
}
pub async fn get_full_account(
    locale: &Locale,
    pool: &Pool,
    account_id: i64,
) -> ServiceResult<FullAccount> {
    let db_account = get_db_account(locale, pool, account_id).await?;
    return Ok(format_account(db_account));
}

pub fn format_account(account: DbAccount) -> FullAccount {
    let cfg = Config::global();
    // todo add auths table
    // get age
    //
    // check

    let now = Utc::now();

    let current_utc_year = now.year();
    let mut age = None;
    if let Some(raw_birthday) = account.birthday {
        let mut tz = FixedOffset::east(cfg.default_timezone_offset_in_seconds);
        if let Some(account_tz) = account.timezone_in_seconds {
            tz = FixedOffset::east(account_tz);
        }
        let birthday_with_tz: Date<FixedOffset> = Date::from_utc(raw_birthday, tz);
        let birthday_utc = birthday_with_tz.naive_utc();
        let birthday_utc_year = birthday_utc.year();
        age = Some(current_utc_year - birthday_utc_year);
    }
    // todo 并行
    let mut actions: Vec<Action> = Vec::new();

    // required info
    if account.birthday_change_count == 0 {
        actions.push(Action {
            _type: ActionType::AddAccountBirthday,
            required: true,
        });
    }
    if account.gender_change_count == 0 {
        actions.push(Action {
            _type: ActionType::AddAccountGender,
            required: true,
        });
    }
    // optional info

    if account.skip_optional_info == false {
        if account.name_change_count == 0 {
            actions.push(Action {
                _type: ActionType::AddAccountName,
                required: false,
            });
        }
        if account.bio_change_count == 0 {
            actions.push(Action {
                _type: ActionType::AddAccountBio,
                required: false,
            });
        }
        if account.profile_image_change_count == 0 {
            actions.push(Action {
                _type: ActionType::AddAccountProfileImage,
                required: false,
            });
        }
    }
    let mut profile_images: Vec<ProfileImage> = Vec::new();
    if let Some(profile_images_value) = account.profile_images {
        let db_profile_images: DbProfileImagesJson = serde_json::from_value(profile_images_value)
            .unwrap_or(DbProfileImagesJson {
                version: JsonVersion::V1,
                images: Vec::new(),
            });
        profile_images = db_profile_images.images;
    }
    FullAccount {
        id: account.id,
        name: account.name,
        bio: account.bio,
        gender: account.gender,
        admin: account.admin,
        moderator: account.moderator,
        vip: account.vip,
        post_count: account.post_count,
        like_count: account.like_count,
        show_age: account.show_age,
        show_distance: account.show_distance,
        suspended: account.suspended,
        suspended_at: account.suspended_at,
        suspended_until: account.suspended_until,
        suspended_reason: account.suspended_reason,
        age: age,
        birthday: account.birthday,
        timezone_in_seconds: account.timezone_in_seconds,
        phone_country_code: account.phone_country_code,
        phone_number: account.phone_number,
        location: account.location,
        country_id: account.country_id,
        state_id: account.state_id,
        profile_images,
        city_id: account.city_id,
        avatar: account.avatar,
        avatar_updated_at: account.avatar_updated_at,
        created_at: account.created_at,
        updated_at: account.updated_at,
        approved: account.approved,
        approved_at: account.approved_at,
        invite_id: account.invite_id,
        actions,
        name_change_count: account.name_change_count,
        bio_change_count: account.bio_change_count,
        birthday_change_count: account.birthday_change_count,
        phone_change_count: account.phone_change_count,
        gender_change_count: account.gender_change_count,
        post_template_count: account.post_template_count,
        show_viewed_action: account.show_viewed_action,
    }
}
pub fn format_account_view(raw: DbAccountView, viewed_by_account: Account) -> AccountView {
    let DbAccountView {
        id,
        created_at,
        updated_at,
        viewed_by,
        target_account_id,
        viewed_count,
    } = raw;
    return AccountView {
        id,
        created_at,
        updated_at,
        viewed_by,
        cursor: id,
        viewed_by_account,
        target_account_id,
        viewed_count,
    };
}
