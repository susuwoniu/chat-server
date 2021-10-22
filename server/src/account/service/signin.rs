use super::get_user::get_user;
use super::util::get_access_token_key;
use crate::account::model::{AuthData, LoginActivityData};
use crate::config::Config;
use crate::error::{Error, ServiceError, ServiceResult};
use crate::middleware::req_meta::ReqMeta;
use crate::types::{KvPool, Pool};
use crate::util::id::next_id;
use crate::util::key_pair::Pair;
use chrono::Utc;
use deadpool_redis::redis::cmd;
use sqlx::query;
// sign in a verified account
pub async fn signin(
  req_meta: ReqMeta,
  login_activity_data: LoginActivityData,
  pool: &Pool,
  kv: &KvPool,
  pair: &Pair,
) -> ServiceResult<AuthData> {
  // lookup account
  let user = get_user(pool, login_activity_data.account_id, req_meta.clone()).await?;

  // if suspended
  if user.suspended {
    return Err(ServiceError::account_suspended(
      &req_meta.locale,
      user.suspended_reason.clone(),
      user.suspended_until.clone(),
      Error::Other(format!("account {} suspened.", user.id)),
    ));
  }
  let login_activity_id = next_id();
  // generate new token
  let now = Utc::now();
  // TODO client id
  let auth_data = AuthData::new(
    login_activity_data.account_id,
    req_meta.client_id,
    pair,
    &Config::get(),
  );
  // add kv token
  // add to kv
  let temp_key = get_access_token_key(auth_data.access_token.clone());
  let mut conn = kv.get().await?;
  cmd("SET")
    .arg(&[
      &temp_key,
      &login_activity_data.account_id.to_string(),
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
      SET signin_count = signin_count + 1,current_signin_at = $1,  last_signin_at=$2
      where id = $3
"#,
    now.naive_utc(),
    login_activity_data.last_signin_at,
    login_activity_data.account_auth_id,
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
    login_activity_data.account_auth_id,
    login_activity_data.account_id,
    req_meta.client_id,
    true
  )
  .execute(&mut tx)
  .await?;
  tx.commit().await?;
  Ok(auth_data)
}
