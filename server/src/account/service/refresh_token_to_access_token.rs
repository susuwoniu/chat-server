use crate::{
  account::{
    model::{AuthData, SigninParam, SigninType},
    service::signin::signin,
    util::get_refresh_token_key,
  },
  alias::{KvPool, Pool},
  error::{Error, ServiceError},
  middleware::{Locale, RefreshTokenAuth},
  types::ServiceResult,
};

use deadpool_redis::redis::cmd;
pub async fn refresh_token_to_access_token(
  locale: &Locale,
  pool: &Pool,
  kv: &KvPool,
  param: &RefreshTokenAuth,
) -> ServiceResult<AuthData> {
  // if redis record exist
  let RefreshTokenAuth {
    account_id,
    client_id,
    device_id,
    ..
  } = param;
  let mut conn = kv.get().await?;
  let temp_key = get_refresh_token_key(*account_id, &device_id);
  let token_option: Option<String> = cmd("GET").arg(&temp_key).query_async(&mut conn).await?;
  if let Some(_) = token_option {
    // yes
    // auth id not need for refresh token
    return signin(
      locale,
      pool,
      kv,
      &SigninParam {
        account_id: *account_id,
        client_id: *client_id,
        account_auth_id: 0,
        device_id: device_id.clone(),
        signin_type: SigninType::RefreshToken,
      },
    )
    .await;
  } else {
    // fail
    return Err(ServiceError::unauthorized(
      &locale,
      "refresh_token_expired",
      Error::Default,
    ));
  }
}
