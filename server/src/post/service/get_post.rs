use crate::{
    account::{model::Account, service::get_account::get_accounts},
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    post::model::{
        DbPost, DbPostFavorite, DbPostView, Post, PostFilter, PostView, PostViewFilter, Sort,
        Visibility,
    },
    types::{DataWithPageInfo, Gender, PageInfo, ServiceResult},
};
use sqlx::query_as;
use std::collections::HashMap;

pub async fn get_posts(
    locale: &Locale,
    pool: &Pool,
    filter: PostFilter,
    auth: Option<Auth>,
    internal: bool,
    is_all_favorite: bool,
) -> ServiceResult<DataWithPageInfo<Post>> {
    let cfg = Config::global();
    let skip = filter.skip.clone();
    let PostFilter {
        longitude,
        latitude,
        distance,
        id,
        ids,
        sort,
        ..
    } = filter;
    let mut default_visibility = Some(Visibility::Public);
    if let Some(auth) = auth.clone() {
        if let Some(filter_account_id) = filter.account_id {
            if auth.account_id == filter_account_id {
                // 用户可以看自己所有的帖子
                default_visibility = None;
            }
        }
    }
    if internal {
        default_visibility = None;
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
    let mut rows = query_as!(DbPost,
    r#"
      select id,time_cursor_change_count,content,background_color,account_id,updated_at,post_template_id,post_template_title,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count,favorite_count,color,CASE WHEN ($32::float8 is null or $33::float8 is null or $34::float8 is null) THEN null ELSE ST_Distance(ST_Transform(ST_SetSRID(ST_Point($32,$33),4326),3857),ST_Transform(geom,3857)) END as distance from posts where 
      ($35::bigint is null or id=$35)
      and ($36::bigint[] is null or id = ANY ($36::bigint[]))
      and ($27::bigint is null or account_id=$27)
      and ($31::bigint is null or post_template_id=$31)
      and ($15::timestamp is null or created_at > $15)
      and ($16::timestamp is null or created_at < $16)
      and ($2::bigint is null or time_cursor > $2) 
      and ($3::bigint is null or time_cursor < $3) 
      and ($4::smallint is null or visibility=$4) 
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
      and ($28::smallint is null or gender =$28)
      and ($29::date is null or birthday >= $29)
      and ($30::date is null or birthday < $30)
      and (CASE WHEN ($32::float8 is null or $33::float8 is null or $34::float8 is null) THEN true ELSE ST_DWithin(geom::geography,ST_SetSRID(ST_Point($32,$33),4326)::geography,$34) END )
      order by CASE WHEN ($37::smallint = 2) THEN favorite_count ELSE time_cursor END desc 
      limit $1
"#,
&limit,
filter.before,
filter.after,
default_visibility as _,
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
filter.gender.clone() as _,
filter.start_birthday,
filter.end_birthday,
filter.post_template_id,
longitude,
latitude,
distance,
id,
ids.as_ref().map(|x| &x[..]),
sort as _
  )
  .fetch_all(pool)
  .await?;

    // remove no access posts
    if internal {
        rows.retain(|row| {
            if row.visibility == Visibility::Private {
                if let Some(auth) = &auth {
                    if auth.account_id == row.account_id || auth.admin || auth.moderator {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                true
            }
        });
    }

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
        .collect::<HashMap<_, _>>();
    let mut favorite_map: HashMap<i64, bool> = HashMap::new();
    if let Some(auth) = auth.clone() {
        if !is_all_favorite {
            let favorites = get_db_favorites(
                locale,
                pool,
                auth.account_id,
                rows.clone().into_iter().map(|row| row.id).collect(),
            )
            .await?;

            favorites.into_iter().for_each(|account| {
                favorite_map.insert(account.post_id, true);
            });
        }
    }

    let data: Vec<Post> = rows
        .into_iter()
        .filter_map(|row| {
            let account = account_map.get(&row.account_id);
            if let Some(account) = account {
                let mut is_favorite = None;
                if is_all_favorite {
                    is_favorite = Some(true);
                } else if auth.is_some() {
                    if let Some(favorite) = favorite_map.get(&row.id) {
                        is_favorite = Some(favorite.clone());
                    } else {
                        is_favorite = Some(false);
                    }
                }

                return Some(format_post(row, account.clone(), auth.clone(), is_favorite));
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
async fn get_db_favorites(
    locale: &Locale,
    pool: &Pool,
    account_id: i64,
    post_ids: Vec<i64>,
) -> ServiceResult<Vec<DbPostFavorite>> {
    let cfg = Config::global();
    if post_ids.len() > cfg.max_page_size as usize {
        return Err(ServiceError::bad_request(
            locale,
            "reach_max_favorites_limit",
            Error::Default,
        ));
    }
    let rows = query_as!(DbPostFavorite,
    r#"
    select id,post_id,created_at,updated_at,account_id,post_account_id from post_favorites  where account_id=$2 and post_id = ANY ($1::bigint[])
"#,
&post_ids,
account_id
  )
  .fetch_all(pool)
  .await?;
    return Ok(rows);
}
async fn _get_is_favorite(
    _locale: &Locale,
    pool: &Pool,
    account_id: i64,
    post_id: i64,
) -> ServiceResult<bool> {
    let rows = query_as!(DbPostFavorite,
    r#"
    select id,post_id,created_at,updated_at,account_id,post_account_id from post_favorites  where account_id=$2 and post_id = $1
"#,
post_id,
account_id
  )
  .fetch_optional(pool)
  .await?;
    Ok(rows.is_some())
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
pub fn format_post(
    raw: DbPost,
    author: Account,
    auth: Option<Auth>,
    is_favorite: Option<bool>,
) -> Post {
    let cfg = Config::global();

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
        post_template_title,
        post_template_id,
        time_cursor,
        target_gender,
        visibility,
        gender,
        client_id: _,
        ip: _,
        color,
        distance,
        time_cursor_change_count,
        favorite_count,
    } = raw;
    let max_time_cursor_change_count = cfg.post.max_time_cursor_change_count;
    let mut is_can_promote = false;
    if let Some(auth) = auth {
        if auth.account_id == author.id && (auth.admin || auth.moderator || auth.vip) {
            if time_cursor_change_count < max_time_cursor_change_count {
                is_can_promote = true;
            }
        }
    }
    return Post {
        is_can_promote: is_can_promote,
        id,
        content,
        background_color,
        account_id,
        updated_at,
        post_template_title,
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
        distance,
        time_cursor_change_count,
        favorite_count,
        is_favorite: is_favorite,
    };
}
fn get_range_value_or_none(range: &Option<&[i64; 2]>, position: usize) -> Option<i64> {
    if let Some(range) = range {
        return Some(range[position]);
    } else {
        return None;
    }
}
