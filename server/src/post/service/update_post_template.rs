use crate::{
  alias::Pool,
  error::{Error, ServiceError},
  middleware::{Auth, Locale},
  post::{
    model::{DbPostTemplate, PostTemplate, UpdatePostTemplateParam},
    service::get_post_template::{format_post_template, get_post_template},
    util,
  },
  types::ServiceResult,
};

use chrono::Utc;
use sqlx::query_as;
pub async fn update_post_template(
  locale: &Locale,
  pool: &Pool,
  id: i64,
  param: UpdatePostTemplateParam,
  auth: Auth,
) -> ServiceResult<PostTemplate> {
  let UpdatePostTemplateParam {
    content,
    featured,
    background_color,
    deleted,
    priority,
  } = param;

  // get post template
  let current = get_post_template(locale, pool, id).await?;

  let now = Utc::now().naive_utc();
  let mut featured_edit_value = None;

  if let Some(content) = content.clone() {
    util::is_post_template_content_valid(locale, &content)?;
  }
  let mut priority_edit_value = None;
  if auth.admin || auth.moderator {
    if let Some(featured_value) = featured {
      featured_edit_value = Some(featured_value);
    }
    if let Some(priority_value) = priority {
      priority_edit_value = Some(priority_value);
    }
  } else {
    // if self
    if auth.account_id != current.account_id {
      return Err(ServiceError::permission_limit(
        locale,
        "only_admin_or_moderator_or_self_can_update_post_template",
        Error::Default,
      ));
    }
  }
  let mut deleted_edit_value = None;
  if let Some(deleted_value) = deleted {
    if deleted_value {
      deleted_edit_value = Some(deleted_value);
    }
  }
  let mut featured_at = None;
  let mut featured_by = None;
  if featured_edit_value.is_some() {
    featured_at = Some(now);
    featured_by = Some(auth.account_id);
  }
  let mut deleted_at = None;
  let mut deleted_by = None;
  if deleted_edit_value.is_some() {
    deleted_at = Some(now);
    deleted_by = Some(auth.account_id);
  }
  let row =  query_as!(DbPostTemplate,
    r#"
UPDATE post_templates set 
content = COALESCE($1,content), 
featured = COALESCE($2,featured),
featured_at = COALESCE($3,featured_at), 
updated_at = $4, 
background_color= COALESCE($5,background_color), 
featured_by = COALESCE($6,featured_by),
deleted = COALESCE($7,deleted), 
deleted_at = COALESCE($8,deleted_at), 
deleted_by = COALESCE($9,deleted_by),
priority = COALESCE($11,priority)
WHERE id = $10 and deleted = false
RETURNING id,content,used_count,skipped_count,background_color,created_at,featured_by,updated_at,account_id,featured,featured_at,time_cursor
"#,
    content,
    featured_edit_value,
    featured_at,
    now,
    background_color,
    featured_by,
    deleted_edit_value,
    deleted_at,
    deleted_by,
    id,
    priority_edit_value
  )
  .fetch_one(pool)
  .await?;

  Ok(format_post_template(row).into())
}
