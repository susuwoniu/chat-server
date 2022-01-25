use crate::{
    account::{
        model::{Account, AccountBlock, AccountLikeFilter, DbAccountBlock},
        service::get_account::get_accounts,
    },
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::Locale,
    types::{DataWithPageInfo, PageInfo, ServiceResult},
};
use sqlx::query_as;
use std::iter::Iterator;

pub async fn get_db_account_blocks(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountLikeFilter,
    target_account_id: Option<i64>,
    account_id: Option<i64>,
) -> ServiceResult<Vec<DbAccountBlock>> {
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
        DbAccountBlock,
        r#"
      select id,account_id,updated_at,created_at,target_account_id from blocks where 
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

pub async fn get_account_blocks_list(
    locale: &Locale,
    pool: &Pool,
    filter: &AccountLikeFilter,
    account_id: i64,
) -> ServiceResult<DataWithPageInfo<AccountBlock>> {
    let rows = get_db_account_blocks(locale, pool, filter, None, Some(account_id)).await?;
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

    let data: Vec<AccountBlock> = rows
        .into_iter()
        .filter_map(|row| {
            let account = account_map.get(&row.target_account_id);
            if let Some(account) = account {
                return Some(format_account_block(row, account.clone()));
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
    let collection = DataWithPageInfo::<AccountBlock> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(collection);
}

pub fn format_account_block(raw: DbAccountBlock, account: Account) -> AccountBlock {
    let DbAccountBlock {
        id,
        created_at,
        updated_at,
        account_id,
        target_account_id,
    } = raw;
    return AccountBlock {
        id,
        created_at,
        updated_at,
        account_id,
        target_account_id,
        target_account: account,
        cursor: id,
    };
}
