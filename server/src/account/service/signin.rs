use super::util::get_access_token_key;
use crate::account::model::{AuthData, LoginActivityData};
use crate::config::Config;
use crate::error::ServiceResult;
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
  let login_activity_id = next_id();
  // generate new token
  let now = Utc::now();
  // TODO client id
  let auth_data = AuthData::new(
    login_activity_data.account_id,
    431242314231,
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
      SET signin_count = signin_count + 1,current_signin_at = $1,  last_signin_at=$2,  refresh_token=$3, refresh_token_expires_at=$4
      where id = $5
"#,
    now.naive_utc(),
    login_activity_data.last_signin_at,
    auth_data.refresh_token,
    auth_data.refresh_token_expires_at,
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
