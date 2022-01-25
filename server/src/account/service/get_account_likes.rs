use crate::{
    account::{
        model::{Account, AccountLike, AccountLikeFilter, AccountLiked, DbAccount, DbAccountLike},
        service::get_account::get_accounts,
    },
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    types::{Action, ActionType, DataWithPageInfo, Gender, JsonVersion, PageInfo, ServiceResult},
};
use chrono::{offset::FixedOffset, Date, Datelike, Duration, NaiveDateTime, Utc};
use itertools::Itertools;
use sqlx::{query, query_as};
use std::iter::Iterator;

pub async fn get_db_account_likes(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountLikeFilter,
    target_account_id: Option<i64>,
    account_id: Option<i64>,
) -> ServiceResult<Vec<DbAccountLike>> {
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
    let rows = query_as!(
        DbAccountLike,
        r#"
      select id,account_id,updated_at,created_at,target_account_id from likes where 
      ($1::bigint is null or target_account_id = $1) 
      and ($7::bigint is null or account_id = $7) 
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
        limit,
        account_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(rows);
}
pub async fn get_account_liked_list(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountLikeFilter,
    target_account_id: i64,
) -> ServiceResult<DataWithPageInfo<AccountLiked>> {
    let rows = get_db_account_likes(locale, pool, filter, Some(target_account_id), None).await?;
    // get accounts info
    let accounts = get_accounts(
        locale,
        pool,
        rows.clone().into_iter().map(|row| row.account_id).collect(),
    )
    .await?;

    let account_map = accounts
        .into_iter()
        .map(|account| (account.id, account))
        .collect::<std::collections::HashMap<_, _>>();

    let data: Vec<AccountLiked> = rows
        .into_iter()
        .filter_map(|row| {
            let account = account_map.get(&row.account_id);
            if let Some(account) = account {
                return Some(format_account_liked(row, account.clone()));
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
    let collection = DataWithPageInfo::<AccountLiked> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(collection);
}
pub async fn get_account_likes_list(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountLikeFilter,
    account_id: i64,
) -> ServiceResult<DataWithPageInfo<AccountLike>> {
    let rows = get_db_account_likes(locale, pool, filter, None, Some(account_id)).await?;
    // get accounts info
    let accounts = get_accounts(
        locale,
        pool,
        rows.clone()
            .into_iter()
            .map(|row| row.target_account_id)
            .collect(),
    )
    .await?;

    let account_map = accounts
        .into_iter()
        .map(|account| (account.id, account))
        .collect::<std::collections::HashMap<_, _>>();

    let data: Vec<AccountLike> = rows
        .into_iter()
        .filter_map(|row| {
            let account = account_map.get(&row.target_account_id);
            if let Some(account) = account {
                return Some(format_account_like(row, account.clone()));
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
    let collection = DataWithPageInfo::<AccountLike> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(collection);
}
pub fn format_account_liked(raw: DbAccountLike, account: Account) -> AccountLiked {
    let DbAccountLike {
        id,
        created_at,
        updated_at,
        account_id,
        target_account_id,
    } = raw;
    return AccountLiked {
        id,
        created_at,
        updated_at,
        account_id,
        target_account_id,
        account: account,
        cursor: id,
    };
}
pub fn format_account_like(raw: DbAccountLike, account: Account) -> AccountLike {
    let DbAccountLike {
        id,
        created_at,
        updated_at,
        account_id,
        target_account_id,
    } = raw;
    return AccountLike {
        id,
        created_at,
        updated_at,
        account_id,
        target_account_id,
        target_account: account,
        cursor: id,
    };
}
