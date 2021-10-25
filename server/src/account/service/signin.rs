use crate::{
  account::{
    model::SigninParam,
    service::get_account::get_account,
    util::{get_access_token_key, AuthData},
  },
  alias::{KvPool, Pool},
  error::{Error, ServiceError, ServiceResult},
  middleware::Locale,
  util::id::next_id,
};

use chrono::Utc;
use deadpool_redis::redis::cmd;
use sqlx::query;
// sign in a verified account
pub async fn signin(
  locale: &Locale,
  pool: &Pool,
  kv: &KvPool,
  param: &SigninParam,
) -> ServiceResult<AuthData> {
  let SigninParam {
    account_auth_id,
    account_id,
    client_id,
  } = param;
  // lookup account
  let user = get_account(pool, account_id, locale).await?;

  // if suspended
  if user.suspended {
    return Err(ServiceError::account_suspended(
      locale,
      user.suspended_reason.clone(),
      user.suspended_until.clone(),
      Error::Other(format!("account {} suspened.", user.id)),
    ));
  }
  let login_activity_id = next_id();
  // generate new token
  let now = Utc::now();
  // TODO client id
  let auth_data = AuthData::new(account_id, client_id);
  // add kv token
  // add to kv
  let temp_key = get_access_token_key(auth_data.access_token.clone());
  let mut conn = kv.get().await?;
  cmd("SET")
    .arg(&[
      &temp_key,
      &account_id.to_string(),
      "PXAT",
      &auth_data.expires_at.timestamp_millis().to_string(),
    ])
    .query_async::<_, ()>(&mut conn)
    .await?;
  // todo  add account auths
  let mut tx = pool.begin().await?;
  // update login record
  query!(
    r#"
      UPDATE account_auths 
      SET signin_count = signin_count + 1,current_signin_at = $1,  last_signin_at=current_signin_at
      where id = $2
"#,
    now.naive_utc(),
    account_auth_id,
  )
  .execute(&mut tx)
  .await?;
  // add login activity
  query!(
    r#"
INSERT INTO login_activities (id,account_auth_id,account_id,client_id,success)
VALUES ($1,$2,$3,$4,$5)
"#,
    login_activity_id,
    account_auth_id,
    account_id,
    client_id,
    true
  )
  .execute(&mut tx)
  .await?;
  tx.commit().await?;
  Ok(auth_data)
}
