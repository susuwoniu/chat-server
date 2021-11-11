use crate::{
  account::{
    model::{FieldOpetation, UpdateAccountParam},
    service::update_account::update_account,
  },
  alias::Pool,
  error::{Error, ServiceError},
  middleware::{Auth, Locale},
  post::{
    model::{PostTemplate, UpdatePostTemplateParam},
    service::get_post_template::get_post_template,
    util,
  },
  types::ServiceResult,
  util::id::next_id,
};

use chrono::Utc;
use sqlx::query;
pub async fn update_post_template(
  locale: &Locale,
  pool: &Pool,
  param: UpdatePostTemplateParam,
  auth: Auth,
) -> ServiceResult<()> {
  let UpdatePostTemplateParam {
    content,
    featured,
    background_color,
    id,
  } = param;
  // get post template
  let current = get_post_template(locale, pool, &id).await?;

  let now = Utc::now().naive_utc();
  let mut featured_edit_value = None;

  if let Some(content) = content.clone() {
    util::is_post_template_content_valid(locale, content)?;
  }

  if auth.admin || auth.moderator {
    if let Some(featured_value) = featured {
      featured_edit_value = Some(featured_value);
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

  let mut featured_at = None;
  if featured_edit_value.is_some() {
    featured_at = Some(now);
  }
  query!(
    r#"
UPDATE post_templates set content = COALESCE($1,content), featured = COALESCE($2,featured), featured_at = COALESCE($3,featured_at), updated_at = $4, background_color= COALESCE($6,background_color)
WHERE id = $5
"#,
    content,
    featured_edit_value,
    featured_at,
    now,
    id,
    background_color,
  )
  .execute(pool)
  .await?;
  // update account post template count
  update_account(
    locale,
    pool,
    UpdateAccountParam {
      post_templates_count: Some(FieldOpetation::IncreaseOne),
      ..Default::default()
    },
    &auth,
  )
  .await?;
  Ok(())
}
