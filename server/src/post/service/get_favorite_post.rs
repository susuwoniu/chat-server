use crate::{
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    post::{
        model::{DbPostFavorite, FavoritePostFilter, Post, PostFavorite, PostFilter},
        service::get_post::get_posts,
    },
    types::{DataWithPageInfo, PageInfo, ServiceResult},
};

use sqlx::query_as;

pub async fn get_favorite_posts(
    locale: &Locale,
    pool: &Pool,
    filter: FavoritePostFilter,
    auth: Option<Auth>,
    _internal: bool,
) -> ServiceResult<DataWithPageInfo<PostFavorite>> {
    let cfg = Config::global();
    let FavoritePostFilter {
        id,
        after,
        before,
        start_time,
        end_time,
        account_id,
        limit: _limit,
    } = filter;

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
        DbPostFavorite,
        r#"
    select id,post_id,created_at,updated_at,account_id,post_account_id from post_favorites where 
    ($6::bigint is null or account_id=$6)
    and ($7::bigint is null or id=$7)
    and ($4::timestamp is null or created_at > $4)
    and ($5::timestamp is null or created_at < $5)
    and ($2::bigint is null or id > $2) 
    and ($3::bigint is null or id < $3) 
    order by id desc 
    limit $1
"#,
        &limit,
        before,
        after,
        start_time,
        end_time,
        account_id,
        id
    )
    .fetch_all(pool)
    .await?;

    // fetch all posts

    let posts = get_posts(
        locale,
        pool,
        PostFilter {
            ids: Some(rows.clone().into_iter().map(|row| row.post_id).collect()),
            ..Default::default()
        },
        auth.clone(),
        false,
    )
    .await?;

    let posts_map = posts
        .data
        .into_iter()
        .map(|post| (post.id, post))
        .collect::<std::collections::HashMap<_, _>>();

    let data: Vec<PostFavorite> = rows
        .into_iter()
        .filter_map(|row| {
            let post = posts_map.get(&row.post_id);
            if let Some(post) = post {
                return Some(format_post_favorite(row, post.clone(), auth.clone()));
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
    let post_collection = DataWithPageInfo::<PostFavorite> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(post_collection);
}

pub fn format_post_favorite(raw: DbPostFavorite, post: Post, auth: Option<Auth>) -> PostFavorite {
    let DbPostFavorite {
        id,
        created_at,
        updated_at,
        post_id,
        post_account_id,
        account_id,
    } = raw;
    return PostFavorite {
        id,
        created_at,
        updated_at,
        account_id,
        post_id,
        post_account_id,
        cursor: id,
        post,
    };
}
