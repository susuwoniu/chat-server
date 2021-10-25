use crate::{
  account::{
    model::{SigninType, SignoutResponseData, SuccessMeta},
    service::get_account::get_account,
    util::{get_refresh_token_key, AuthData},
  },
  alias::{KvPool, Pool},
  error::{Error, ServiceError, ServiceResult},
  middleware::{Auth, Locale},
  util::id::next_id,
};

use chrono::Utc;
use deadpool_redis::redis::cmd;
use sqlx::query;
// sign in a verified account
pub async fn signout(
  locale: &Locale,
  kv: &KvPool,
  param: &Auth,
) -> ServiceResult<SignoutResponseData> {
  let Auth {
    account_id,
    device_id,
    ..
  } = param;
  // add refresh token to kv
  // add to kv
  let temp_key = get_refresh_token_key(*account_id, &device_id);
  let mut conn = kv.get().await?;
  cmd("DEL")
    .arg(&[&temp_key])
    .query_async::<_, ()>(&mut conn)
    .await?;
  // if not refresh token , so write

  Ok(SignoutResponseData {
    meta: SuccessMeta { ok: true },
  })
}
