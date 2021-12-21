use crate::{
    account::{
        model::Account,
        service::get_account::{get_account, get_accounts},
    },
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    post::model::{DbPost, DbPostView, Post, PostFilter, PostView, PostViewFilter, Visibility},
    types::{DataWithPageInfo, Gender, PageInfo, ServiceResult},
};

use sqlx::query_as;

pub async fn get_posts(
    locale: &Locale,
    pool: &Pool,
    filter: &PostFilter,
    auth: &Option<Auth>,
) -> ServiceResult<DataWithPageInfo<Post>> {
    let cfg = Config::global();
    let skip = filter.skip.clone();
    let mut default_visibility = Some(Visibility::Public);
    if let Some(ref auth) = auth {
        if let Some(filter_account_id) = filter.account_id {
            if auth.account_id == filter_account_id {
                // 用户可以看自己所有的帖子
                default_visibility = None;
            }
        }
    }
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
    let rows = query_as!(DbPost,
    r#"
      select id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count,color from posts where 
      ($27::bigint is null or account_id=$27)
      and ($15::timestamp is null or created_at > $15)
      and ($16::timestamp is null or created_at < $16)
      and ($2::bigint is null or time_cursor > $2) 
      and ($3::bigint is null or time_cursor < $3) 
      and ($4::visibility is null or visibility=$4) 
      and approved=true 
      and deleted=false 
      and ($5::bigint is null or time_cursor > $5 or time_cursor < $6)
      and ($7::bigint is null or time_cursor > $7 or time_cursor < $8)
      and ($9::bigint is null or time_cursor > $9 or time_cursor < $10)
      and ($11::bigint is null or time_cursor > $11 or time_cursor < $12)
      and ($13::bigint is null or time_cursor > $13 or time_cursor < $14)
      and ($17::bigint is null or time_cursor > $17 or time_cursor < $18)
      and ($19::bigint is null or time_cursor > $19 or time_cursor < $20)
      and ($21::bigint is null or time_cursor > $21 or time_cursor < $22)
      and ($23::bigint is null or time_cursor > $23 or time_cursor < $24)
      and ($25::bigint is null or time_cursor > $25 or time_cursor < $26)
      and ($28::gender is null or gender =$28)
      and ($29::date is null or birthday >= $29)
      and ($30::date is null or birthday < $30)
      order by time_cursor desc 
      limit $1
"#,
&limit,
filter.before,
filter.after,
default_visibility as Option<Visibility>,
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
filter.start_time,
filter.end_time,
get_range_value_or_none(&skip.get(5),0),
get_range_value_or_none(&skip.get(5),1),
get_range_value_or_none(&skip.get(6),0),
get_range_value_or_none(&skip.get(6),1),
get_range_value_or_none(&skip.get(7),0),
get_range_value_or_none(&skip.get(7),1),
get_range_value_or_none(&skip.get(8),0),
get_range_value_or_none(&skip.get(8),1),
get_range_value_or_none(&skip.get(9),0),
get_range_value_or_none(&skip.get(9),1),
filter.account_id,
filter.gender.clone() as Option<Gender>,
filter.start_birthday,
filter.end_birthday
  )
  .fetch_all(pool)
  .await?;

    // fetch all account

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

pub async fn get_post_views(
    locale: &Locale,
    pool: &Pool,
    filter: &PostViewFilter,
    post_id: i64,
) -> ServiceResult<DataWithPageInfo<PostView>> {
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
        DbPostView,
        r#"
      select id,created_at,updated_at,viewed_by,post_id,post_account_id from post_view where 
      post_id=$2
      and ($3::bigint is null or id > $3) 
      and ($4::bigint is null or id < $4) 
      and ($5::bigint is null or post_account_id=$5)
      and ($6::timestamp is null or created_at > $6)
      and ($7::timestamp is null or created_at < $7)
      order by id desc 
      limit $1
"#,
        &limit,
        post_id,
        filter.before,
        filter.after,
        filter.post_account_id,
        filter.start_time,
        filter.end_time,
    )
    .fetch_all(pool)
    .await?;

    // fetch all account

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

    let data: Vec<PostView> = rows
        .into_iter()
        .filter_map(|row| {
            let account = account_map.get(&row.viewed_by);
            if let Some(account) = account {
                return Some(format_post_view(row, account.clone()));
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
    let collection = DataWithPageInfo::<PostView> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(collection);
}
// 获取文章
pub async fn get_post(
    locale: &Locale,
    pool: &Pool,
    id: i64,
    auth: &Option<Auth>,
) -> ServiceResult<Post> {
    let mut has_permission_view = false;

    let row = query_as!(DbPost,
    r#"
      select id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count,color from posts where id=$1 and deleted=false
"#,
id
  )
  .fetch_optional(pool)
  .await?;
    if let Some(row) = row {
        // is public
        if row.visibility == Visibility::Public {
            has_permission_view = true;
        } else if let Some(ref auth) = auth {
            if auth.account_id == row.account_id {
                // 用户可以看自己所有的帖子
                has_permission_view = true;
            }
        }
        // 没有权限则拒绝
        if !has_permission_view {
            return Err(ServiceError::permission_limit(
                locale,
                "no_permission_to_view_post",
                Error::Other(format!("Can not view not public post {}", id)),
            ));
        }
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
pub fn format_post_view(raw: DbPostView, viewed_by_account: Account) -> PostView {
    let DbPostView {
        id,
        created_at,
        updated_at,
        viewed_by,
        post_id,
        post_account_id,
    } = raw;
    return PostView {
        id,
        created_at,
        updated_at,
        viewed_by,
        post_id,
        post_account_id,
        cursor: id,
        viewed_by_account,
    };
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
        color,
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
        color,
    };
}
fn get_range_value_or_none(range: &Option<&[i64; 2]>, position: usize) -> Option<i64> {
    if let Some(range) = range {
        return Some(range[position]);
    } else {
        return None;
    }
}
