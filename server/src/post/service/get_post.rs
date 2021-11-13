use crate::{
  account::{
    model::{Account, UpdateAccountParam},
    service::{
      get_account::{get_account, get_accounts},
      update_account::update_account,
    },
  },
  alias::Pool,
  error::{Error, ServiceError},
  global::Config,
  middleware::{Auth, Locale},
  post::{
    model::{DbPost, Post, PostFilter, Visibility},
    service::get_post_template::get_post_template,
    util,
  },
  types::{DataWithPageInfo, Gender, PageInfo, Range, ServiceResult},
  util::{id::next_id, string::parse_skip_range},
};
use chrono::Utc;
use ipnetwork17::IpNetwork;
use sqlx::query_as;

pub async fn get_posts(
  locale: &Locale,
  pool: &Pool,
  filter: &PostFilter,
) -> ServiceResult<DataWithPageInfo<Post>> {
  let cfg = Config::global();
  let skip = filter.skip.clone();
  let rows = query_as!(DbPost,
    r#"
      select id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count from posts where 
      ($2::bigint is null or time_cursor > $2) 
      and ($3::bigint is null or time_cursor < $3) 
      and visibility=$4 
      and approved=true 
      and deleted=false 
      and ($5::bigint is null or time_cursor > $5 or time_cursor < $6)
      and ($7::bigint is null or time_cursor > $7 or time_cursor < $8)
      and ($9::bigint is null or time_cursor > $9 or time_cursor < $10)
      and ($11::bigint is null or time_cursor > $11 or time_cursor < $12)
      and ($13::bigint is null or time_cursor > $13 or time_cursor < $14)
      order by time_cursor desc 
      limit $1
"#,
&cfg.page_size,
filter.before,
filter.after,
Visibility::Public as Visibility,
get_range_value_or_none(&skip.get(0),0),
get_range_value_or_none(&skip.get(0),1),
get_range_value_or_none(&skip.get(1),0),
get_range_value_or_none(&skip.get(1),1),
get_range_value_or_none(&skip.get(2),0),
get_range_value_or_none(&skip.get(2),1),
get_range_value_or_none(&skip.get(3),0),
get_range_value_or_none(&skip.get(3),1),
get_range_value_or_none(&skip.get(4),0),
get_range_value_or_none(&skip.get(4),1),
  )
  .fetch_all(pool)
  .await?;

  // fetch all account

  let accounts = get_accounts(
    locale,
    pool,
    &rows.clone().into_iter().map(|row| row.account_id).collect(),
  )
  .await?;

  let account_map = accounts
    .into_iter()
    .map(|account| (account.id, account))
    .collect::<std::collections::HashMap<_, _>>();

  let data: Vec<Post> = rows
    .into_iter()
    .filter_map(|row| {
      let account = account_map.get(&row.account_id);
      if let Some(account) = account {
        return Some(format_post(row, account.clone()));
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
  let post_collection = DataWithPageInfo::<Post> {
    data,
    page_info: PageInfo { start, end },
  };
  return Ok(post_collection);
}

pub async fn get_post(locale: &Locale, pool: &Pool, id: i64) -> ServiceResult<Post> {
  let row = query_as!(DbPost,
    r#"
      select id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count from posts where id=$1 and deleted=false
"#,
id
  )
  .fetch_optional(pool)
  .await?;
  if let Some(row) = row {
    let account = get_account(locale, pool, row.account_id).await?;

    return Ok(format_post(row, account.into()));
  } else {
    return Err(ServiceError::record_not_exist(
      locale,
      "post_not_exists",
      Error::Other(format!("Can not found post template id: {} at db", id)),
    ));
  }
}
pub fn format_post(raw: DbPost, author: Account) -> Post {
  let DbPost {
    id,
    content,
    background_color,
    account_id,
    updated_at,
    created_at,
    skipped_count,
    replied_count,
    viewed_count,
    post_template_id,
    time_cursor,
    target_gender,
    visibility,
    gender,
    client_id: _,
    ip: _,
  } = raw;
  return Post {
    id,
    content,
    background_color,
    account_id,
    updated_at,
    created_at,
    skipped_count,
    replied_count,
    viewed_count,
    post_template_id,
    cursor: time_cursor,
    target_gender,
    gender,
    visibility,
    author: author,
  };
}
fn get_range_value_or_none(range: &Option<&[i64; 2]>, position: usize) -> Option<i64> {
  if let Some(range) = range {
    return Some(range[position]);
  } else {
    return None;
  }
}
