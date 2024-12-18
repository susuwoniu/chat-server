use crate::{
    account::model::{
        Account, AccountView, AccountViewFilter, DbAccount, DbAccountView, DbAvatarJson,
        FullAccount,
    },
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    types::{Action, ActionType, DataWithPageInfo, Gender, PageInfo, ServiceResult},
    util::image::format_images,
};
use chrono::{offset::FixedOffset, Date, Datelike, Duration, NaiveDateTime, Utc};
use itertools::Itertools;
use sqlx::{query, query_as};
use std::iter::Iterator;
pub async fn get_db_account(
    locale: &Locale,
    pool: &Pool,
    account_id: i64,
) -> ServiceResult<DbAccount> {
    let row=  query_as!(DbAccount,
    r#"
      select id,name,bio,favorite_count,gender as "gender:Gender",admin,moderator,vip,post_count,like_count,show_age,show_distance,show_viewed_action,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id,name_change_count,bio_change_count,gender_change_count,birthday_change_count,phone_change_count,gender_updated_at,profile_image_change_count,post_template_count,profile_images,last_post_created_at,agree_community_rules_at,bio_updated_at,name_updated_at,avatar_change_count,birthday_updated_at,phone_updated_at from accounts where id = $1 and deleted=false
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
      select id,viewed_by,viewed_count,updated_at,created_at,target_account_id,time_cursor from account_views where 
      target_account_id = $1 
      and ($2::bigint is null or time_cursor > $2) 
      and ($3::bigint is null or time_cursor < $3) 
      and ($4::timestamp is null or created_at > $4)
      and ($5::timestamp is null or created_at < $5)
      order by time_cursor desc 
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
      select id,viewed_by,viewed_count,updated_at,created_at,target_account_id,time_cursor from account_views where target_account_id = $1 and viewed_by = $2
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
      select id,name,bio,gender as "gender:Gender",admin,moderator,vip,post_count,like_count,show_age,show_distance,favorite_count,show_viewed_action,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id,name_change_count,bio_change_count,gender_change_count,birthday_change_count,phone_change_count,gender_updated_at,profile_image_change_count,post_template_count,profile_images,last_post_created_at,agree_community_rules_at,bio_updated_at,name_updated_at,avatar_change_count,birthday_updated_at,phone_updated_at from accounts where id = ANY ($1::bigint[]) and deleted=false
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
#[allow(dead_code)]
pub async fn get_account(locale: &Locale, pool: &Pool, account_id: i64) -> ServiceResult<Account> {
    let db_account = get_db_account(locale, pool, account_id).await?;
    return Ok(Account::from(format_account(db_account)));
}
pub async fn get_other_account(
    locale: &Locale,
    pool: &Pool,
    account_id: i64,
    auth: Option<Auth>,
) -> ServiceResult<Account> {
    let db_account = get_db_account(locale, pool, account_id).await?;
    // get is_liked if auth
    let mut account = Account::from(format_account(db_account));

    if let Some(auth) = auth {
        // get is_liked
        let is_liked = Some(get_is_liked(locale, pool, auth.account_id, account_id).await?);
        account.is_liked = is_liked;

        let is_blocked = Some(get_is_blocked(locale, pool, auth.account_id, account_id).await?);
        account.is_blocked = is_blocked;
    }
    return Ok(account);
}
pub async fn get_is_liked(
    _: &Locale,
    pool: &Pool,
    account_id: i64,
    target_account_id: i64,
) -> ServiceResult<bool> {
    let row = query!(
        r#"
          select id from likes where account_id = $1 and target_account_id=$2
    "#,
        account_id,
        target_account_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(_) = row {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub async fn get_is_blocked(
    _: &Locale,
    pool: &Pool,
    account_id: i64,
    target_account_id: i64,
) -> ServiceResult<bool> {
    let row = query!(
        r#"
          select id from blocks where account_id = $1 and target_account_id=$2
    "#,
        account_id,
        target_account_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(_) = row {
        return Ok(true);
    } else {
        return Ok(false);
    }
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
    // first add aggre rule page action

    if account.agree_community_rules_at.is_none() {
        let rules = include_str!("../../../../resources/terms/community-rules.md");
        actions.push(Action {
            _type: ActionType::AgreeCommunityRules,
            required: true,
            content: Some(rules.to_string()),
        });
    }

    // required info
    if account.birthday_change_count == 0 {
        actions.push(Action {
            _type: ActionType::AddAccountBirthday,
            required: true,
            content: None,
        });
    }
    if account.gender_change_count == 0 {
        actions.push(Action {
            _type: ActionType::AddAccountGender,
            required: true,
            content: None,
        });
    }
    // optional info

    if account.name_change_count == 0 {
        actions.push(Action {
            _type: ActionType::AddAccountName,
            required: true,
            content: None,
        });
    }

    if account.avatar_change_count == 0 && account.avatar_updated_at.is_none() {
        actions.push(Action {
            _type: ActionType::AddAccountAvatar,
            required: true,
            content: None,
        });
    }
    // if account.bio_change_count == 0 && account.bio_updated_at.is_none() {
    //     actions.push(Action {
    //         _type: ActionType::AddAccountBio,
    //         required: false,
    //         content: None,
    //     });
    // }

    let profile_images = format_images(account.profile_images);

    let mut avatar = None;
    if let Some(avatar_value) = account.avatar {
        let db_avatar: DbAvatarJson = serde_json::from_value(avatar_value).unwrap();
        avatar = Some(db_avatar.image);
    }
    let min_duration_between_posts_in_minutes = cfg.post.min_duration_between_posts_in_minutes;
    let vip_min_duration_between_posts_in_minutes =
        cfg.post.vip_min_duration_between_posts_in_minutes;
    let last_post_created_at = account.last_post_created_at;
    let mut next_post_not_before = NaiveDateTime::from_timestamp(0, 0);

    if let Some(last_post_created_at) = last_post_created_at {
        if account.vip || account.admin || account.admin {
            next_post_not_before =
                last_post_created_at + Duration::minutes(vip_min_duration_between_posts_in_minutes);
        } else {
            next_post_not_before =
                last_post_created_at + Duration::minutes(min_duration_between_posts_in_minutes);
        }
    }
    let now_naive = now.naive_utc();

    // is can post next post
    let is_can_post = now_naive >= next_post_not_before;
    // next post seconds
    let next_post_in_seconds = next_post_not_before - now_naive;
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
        avatar: avatar,
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
        last_post_created_at: account.last_post_created_at,
        next_post_not_before: next_post_not_before,
        is_can_post: is_can_post,
        now: now,
        next_post_in_seconds: next_post_in_seconds.num_seconds(),
        agree_community_rules_at: account.agree_community_rules_at,
        bio_updated_at: account.bio_updated_at,
        name_updated_at: account.name_updated_at,
        favorite_count: account.favorite_count,
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
        time_cursor,
    } = raw;
    return AccountView {
        id,
        created_at,
        updated_at,
        viewed_by,
        cursor: time_cursor,
        viewed_by_account,
        target_account_id,
        viewed_count,
    };
}
