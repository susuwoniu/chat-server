use crate::{
  alias::Pool,
  error::{Error, ServiceError},
  global::Config,
  middleware::Locale,
  post::model::{
    CreatePostTemplateParam, DbPostTemplate, FullPostTemplate, PostTemplate, PostTemplateFilter,
  },
  types::{Action, ActionType, ServiceResult},
};
use chrono::offset::FixedOffset;
use chrono::Datelike;
use chrono::{Date, Utc};
use sqlx::{query, query_as};

pub async fn get_full_post_templates(
  locale: &Locale,
  pool: &Pool,
  filter: &PostTemplateFilter,
) -> ServiceResult<Vec<FullPostTemplate>> {
  let cfg = Config::global();
  let mut limit = cfg.page_size;
  if let Some(featured) = filter.featured {
    if featured {
      // featured 默认200
      limit = 200
    }
  }
  let rows = query_as!(DbPostTemplate,
      r#"
        select id,content,used_count,skipped_count,background_color,created_at,featured_by,updated_at,account_id,featured,featured_at from post_templates where  ($2::bigint is null or id > $2) and ($3::bigint is null or id < $3) and ($4::bool is null or featured = $4) and deleted=false  order by id desc limit $1
  "#,
  limit ,
  filter.after,
  filter.before,
  filter.featured
    )
    .fetch_all(pool)
    .await?;
  return Ok(
    rows
      .into_iter()
      .map(|row| format_post_template(row))
      .collect(),
  );
}
pub async fn get_post_templates(
  locale: &Locale,
  pool: &Pool,
  filter: &PostTemplateFilter,
) -> ServiceResult<Vec<PostTemplate>> {
  return Ok(
    get_full_post_templates(locale, pool, filter)
      .await?
      .into_iter()
      .map(|row| PostTemplate::from(row))
      .collect(),
  );
}

pub async fn get_full_post_template(
  locale: &Locale,
  pool: &Pool,
  id: i64,
) -> ServiceResult<FullPostTemplate> {
  let row = query_as!(DbPostTemplate,
    r#"
      select id,content,used_count,skipped_count,background_color,created_at,featured_by,updated_at,account_id,featured,featured_at from post_templates where id=$1 and deleted=false
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
    content: row.content,
    used_count: row.used_count,
    skipped_count: row.skipped_count,
    background_color: row.background_color,
    account_id: row.account_id,
    featured: row.featured,
    featured_at: row.featured_at,
    created_at: row.created_at,
    updated_at: row.updated_at,
    featured_by: row.featured_by,
  };
}
