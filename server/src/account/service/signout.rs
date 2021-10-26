use crate::{
  account::{
    model::{SuccessMeta, SuccessResponseData},
    util::get_refresh_token_key,
  },
  alias::KvPool,
  error::ServiceResult,
  middleware::{Auth, Locale},
};
use deadpool_redis::redis::cmd;
// sign in a verified account
pub async fn signout(
  locale: &Locale,
  kv: &KvPool,
  param: &Auth,
) -> ServiceResult<SuccessResponseData> {
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

  Ok(SuccessResponseData {
    meta: SuccessMeta { ok: true },
  })
}
