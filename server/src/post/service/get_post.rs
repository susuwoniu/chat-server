use crate::{
  account::{
    model::{FieldOpetation, SlimAccount, UpdateAccountParam},
    service::{get_account::get_account, update_account::update_account},
  },
  alias::Pool,
  error::{Error, ServiceError},
  middleware::{Auth, Locale},
  post::{
    model::{CreatePostParam, DbPost, Post, Visibility},
    service::get_post_template::get_post_template,
    util,
  },
  types::{Gender, ServiceResult},
  util::id::next_id,
};
use chrono::Utc;
use ipnetwork17::IpNetwork;
use sqlx::query_as;
pub async fn get_post(locale: &Locale, pool: &Pool, id: i64) -> ServiceResult<Post> {
  let row = query_as!(DbPost,
    r#"
      select id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skip_count,view_count from posts where id=$1 and deleted=false
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
pub fn format_post(raw: DbPost, author: SlimAccount) -> Post {
  let DbPost {
    id,
    content,
    background_color,
    account_id,
    updated_at,
    created_at,
    skip_count,
    view_count,
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
    skip_count,
    view_count,
    post_template_id,
    time_cursor,
    target_gender,
    gender,
    visibility,
    author: author,
  };
}
