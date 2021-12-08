use crate::{
  alias::KvPool,
  constant::IM_SERVER_ADMIN_TOKEN_TEMP_KEY,
  global::Config,
  im::{model::ImCreateTokenParam, service::create_im_token::create_im_token},
  middleware::Locale,
  types::ServiceResult,
};

use chrono::Utc;
use deadpool_redis::redis::cmd;
use std::sync::atomic::{AtomicBool, Ordering};
static IS_GETTING_ADMIN_TOKEN: AtomicBool = AtomicBool::new(false);
pub async fn get_or_create_admin_im_token(kv: &KvPool) -> ServiceResult<String> {
  let cfg = Config::global();
  let now = Utc::now();
  let mut conn = kv.get().await?;
  let admin_token: Option<String> = cmd("GET")
    .arg(&IM_SERVER_ADMIN_TOKEN_TEMP_KEY)
    .query_async(&mut conn)
    .await?;
  if let Some(admin_token) = admin_token {
    let admin_token_ttl: Option<i64> = cmd("PTTL")
      .arg(&IM_SERVER_ADMIN_TOKEN_TEMP_KEY)
      .query_async(&mut conn)
      .await?;
    if let Some(admin_token_ttl) = admin_token_ttl {
      // check expire, if expire < 30 min,then create new one

      if admin_token_ttl > 0 && IS_GETTING_ADMIN_TOKEN.load(Ordering::Relaxed) == false {
        IS_GETTING_ADMIN_TOKEN.store(true, Ordering::Relaxed);
        create_im_admin_token(kv).await?;
        IS_GETTING_ADMIN_TOKEN.store(false, Ordering::Relaxed);
      }
    }
    return Ok(admin_token);
  } else {
    // create
    return create_im_admin_token(kv).await;
  }
}

pub async fn create_im_admin_token(kv: &KvPool) -> ServiceResult<String> {
  let cfg = Config::global();

  let im_token_data = create_im_token(
    &Locale::default(),
    ImCreateTokenParam {
      account_id: cfg.im.admin_account_id,
      try_signup: true,
      platform: 8,
      name: "admin".to_string(),
      avatar: None,
      now: Utc::now(),
    },
  )
  .await?;
  let mut conn = kv.get().await?;

  cmd("SET")
    .arg(&[
      &IM_SERVER_ADMIN_TOKEN_TEMP_KEY,
      im_token_data.im_access_token.as_str(),
      "PXAT",
      &im_token_data
        .im_access_token_expires_at
        .timestamp_millis()
        .to_string(),
    ])
    .query_async::<_, ()>(&mut conn)
    .await?;

  return Ok(im_token_data.im_access_token);
}
