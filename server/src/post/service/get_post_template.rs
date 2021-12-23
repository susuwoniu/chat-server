use crate::{
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::Locale,
    post::model::{DbPostTemplate, FullPostTemplate, PostTemplate, PostTemplateFilter},
    types::{DataWithPageInfo, PageInfo, ServiceResult},
};

use sqlx::query_as;
pub async fn get_full_post_templates(
    locale: &Locale,
    pool: &Pool,
    filter: &PostTemplateFilter,
) -> ServiceResult<DataWithPageInfo<FullPostTemplate>> {
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
    } else {
        if let Some(featured) = filter.featured {
            if featured {
                // featured 默认200
                limit = 200
            }
        }
    }
    let rows = query_as!(DbPostTemplate,
      r#"
        select id,title,content,used_count,skipped_count,created_at,featured_by,updated_at,account_id,featured,time_cursor,featured_at from post_templates where  ($2::bigint is null or time_cursor < $2) and ($3::bigint is null or time_cursor > $3) and ($4::bool is null or featured = $4) and deleted=false  order by priority,time_cursor desc limit $1
  "#,
  limit ,
  filter.after,
  filter.before,
  filter.featured
    )
    .fetch_all(pool)
    .await?;
    let data: Vec<FullPostTemplate> = rows
        .into_iter()
        .map(|row| format_post_template(row))
        .collect();
    let mut start = None;
    let mut end = None;
    if let Some(row) = data.first() {
        start = Some(row.cursor);
    }
    if let Some(row) = data.last() {
        end = Some(row.cursor);
    }
    let post_collection = DataWithPageInfo::<FullPostTemplate> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(post_collection);
}
pub async fn get_post_templates(
    locale: &Locale,
    pool: &Pool,
    filter: &PostTemplateFilter,
) -> ServiceResult<DataWithPageInfo<PostTemplate>> {
    let full_post = get_full_post_templates(locale, pool, filter).await?;

    let posts = full_post
        .data
        .into_iter()
        .map(|row| PostTemplate::from(row))
        .collect();
    return Ok(DataWithPageInfo::<PostTemplate> {
        data: posts,
        page_info: full_post.page_info,
    });
}

pub async fn get_full_post_template(
    locale: &Locale,
    pool: &Pool,
    id: i64,
) -> ServiceResult<FullPostTemplate> {
    let row = query_as!(DbPostTemplate,
    r#"
      select id,title,content,used_count,skipped_count,created_at,featured_by,updated_at,account_id,featured,featured_at,time_cursor from post_templates where id=$1 and deleted=false
"#,
id
  )
  .fetch_optional(pool)
  .await?;
    if let Some(row) = row {
        return Ok(format_post_template(row));
    } else {
        return Err(ServiceError::record_not_exist(
            locale,
            "post_template_not_exists",
            Error::Other(format!("Can not found post template id: {} at db", id)),
        ));
    }
}

pub async fn get_post_template(
    locale: &Locale,
    pool: &Pool,
    id: i64,
) -> ServiceResult<PostTemplate> {
    return Ok(get_full_post_template(locale, pool, id).await?.into());
}

pub fn format_post_template(row: DbPostTemplate) -> FullPostTemplate {
    return FullPostTemplate {
        id: row.id,
        title: row.title,
        content: row.content,
        used_count: row.used_count,
        skipped_count: row.skipped_count,
        account_id: row.account_id,
        featured: row.featured,
        featured_at: row.featured_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
        featured_by: row.featured_by,
        cursor: row.time_cursor,
    };
}
