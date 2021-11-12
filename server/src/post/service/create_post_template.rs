use crate::{
  account::{model::UpdateAccountParam, service::update_account::update_account},
  alias::Pool,
  middleware::{Auth, Locale},
  post::{
    model::{CreatePostTemplateParam, PostTemplate},
    util,
  },
  types::{FieldAction, ServiceResult},
  util::id::next_id,
};
use ipnetwork17::IpNetwork;

use chrono::Utc;
use sqlx::query;
pub async fn create_post_template(
  locale: &Locale,
  pool: &Pool,
  param: CreatePostTemplateParam,
  auth: Auth,
  ip: IpNetwork,
) -> ServiceResult<PostTemplate> {
  // add post template
  let id = next_id();
  let now = Utc::now().naive_utc();
  let mut featured = false;

  if auth.admin || auth.moderator {
    if let Some(featured_value) = param.featured {
      featured = featured_value;
    }
  }

  let mut featured_at = None;
  if featured {
    featured_at = Some(now);
  }
  // TODO check param is valid

  util::is_post_template_content_valid(locale, param.content.clone())?;

  query!(
    r#"
INSERT INTO post_templates (id,content,background_color,account_id,updated_at,featured,featured_at,ip)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
"#,
    id,
    param.content,
    param.background_color,
    auth.account_id,
    now,
    featured,
    featured_at,
    ip,
  )
  .execute(pool)
  .await?;
  // update account post template count
  update_account(
    locale,
    pool,
    UpdateAccountParam {
      post_template_count_action: Some(FieldAction::IncreaseOne),
      ..Default::default()
    },
    &auth,
  )
  .await?;
  return Ok(PostTemplate {
    id,
    content: param.content,
    background_color: param.background_color,
    account_id: auth.account_id,
    updated_at: now,
    created_at: now,
    featured: featured,
    featured_at: featured_at,
    used_count: 0,
    skipped_count: 0,
  });
}
