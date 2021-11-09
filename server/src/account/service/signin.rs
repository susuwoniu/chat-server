use crate::{
  account::{
    model::{SigninParam, SigninType},
    service::get_account::get_account,
    util::{get_refresh_token_key, AuthData},
  },
  alias::{KvPool, Pool},
  error::{Error, ServiceError},
  middleware::Locale,
  types::ServiceResult,
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
    signin_type,
    device_id,
  } = param;
  // lookup account
  let account = get_account(locale, pool, account_id).await?;
  // if suspended
  if account.suspended {
    return Err(ServiceError::account_suspended(
      locale,
      account.suspended_reason.clone(),
      account.suspended_until.clone(),
      Error::Other(format!("account {} suspened.", account.id)),
    ));
  }
  let login_activity_id = next_id();
  // generate new token
  let now = Utc::now();
  let mut roles: Vec<String> = Vec::new();
  if account.admin {
    roles.push("admin".to_string());
  }
  if account.moderator {
    roles.push("moderator".to_string());
  }
  if account.vip {
    roles.push("vip".to_string());
  }
  // TODO client id
  let auth_data = AuthData::new(
    account_id,
    client_id,
    device_id.to_string(),
    roles,
    account.actions,
  );
  // add refresh token to kv
  // add to kv
  let temp_key = get_refresh_token_key(*account_id, &auth_data.device_id);
  let mut conn = kv.get().await?;
  cmd("SET")
    .arg(&[
      &temp_key,
      "1",
      "PXAT",
      &auth_data
        .refresh_token_expires_at
        .timestamp_millis()
        .to_string(),
    ])
    .query_async::<_, ()>(&mut conn)
    .await?;
  // if not refresh token , so write
  if signin_type != &SigninType::RefreshToken {
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
  }

  Ok(auth_data)
}
