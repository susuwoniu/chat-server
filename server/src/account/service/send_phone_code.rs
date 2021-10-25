use crate::{
  account::{
    model::{PhoneCodeMeta, PhoneCodeResponseData, SendPhoneCodePathParam},
    util::get_phone_code_temp_key,
  },
  alias::KvPool,
  error::{Error, ServiceError, ServiceResult},
  global::{Config, I18n, ENV},
  middleware::Locale,
  util::random::get_randome_code,
};
use axum::{
  extract::{Extension, Path},
  Json,
};
use deadpool_redis::redis::cmd;
use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};

pub async fn send_phone_code(
  path_param: SendPhoneCodePathParam,
  kv: &KvPool,
  locale: &Locale,
) -> ServiceResult<PhoneCodeResponseData> {
  // verify code
  // get random code
  let cfg = Config::global();
  let code = if cfg.env == ENV::Dev {
    "123456".to_string()
  } else {
    get_randome_code()
  };

  // add to kv
  let temp_key = get_phone_code_temp_key(&path_param.phone_country_code, &path_param.phone_number);
  let mut conn = kv.get().await?;

  // check time, if duration 1 minutes
  let code_option: Option<i64> = cmd("TTL").arg(&temp_key).query_async(&mut conn).await?;
  let ttl = code_option.unwrap_or(-2);
  // last send offset
  let offset_since_last_send_in_seconds =
    cfg.auth.phone_code_verification_expires_in_minutes * 60 - ttl;
  if offset_since_last_send_in_seconds >= 0
    && offset_since_last_send_in_seconds < cfg.auth.phone_code_verification_duration_in_seconds
  {
    // try later
    return Err(ServiceError::get_phone_code_too_many_requests(
      &locale,
      Error::Other(format!("ttl: {}, path: {:?}", ttl, &path_param)),
    ));
  }
  cmd("SET")
    .arg(&[
      &temp_key,
      &code,
      "EX",
      &(cfg.auth.phone_code_verification_expires_in_minutes * 60).to_string(),
    ])
    .query_async::<_, ()>(&mut conn)
    .await?;
  let mut args = FluentArgs::new();
  args.set("code", code.clone());
  let i18n = I18n::global();
  let phone_text = &i18n.with_args("phone-verify-code-template", &locale, args);
  tracing::info!("Phone text: {}", phone_text);
  // todo send sms
  // todo add auths table
  Ok(PhoneCodeResponseData {
    meta: PhoneCodeMeta {
      length: code.len(),
      expires_in_minutes: cfg.auth.phone_code_verification_expires_in_minutes,
    },
  })
}
