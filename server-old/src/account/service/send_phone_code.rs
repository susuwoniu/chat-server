use super::util::get_phone_code_temp_key;
use crate::account::model::{PhoneCodeMeta, PhoneCodePostData, PhoneCodeResponseData};
use crate::config::Config;
use crate::config::ENV;
use crate::error::{Error, ServiceError, ServiceResult};
use crate::i18n::I18N;
use crate::middleware::req_meta::ReqMeta;
use crate::types::KvPool;
use crate::util::random::get_randome_code;
use deadpool_redis::redis::cmd;
use fluent_bundle::FluentArgs;
pub async fn send_phone_code(
  req_meta: ReqMeta,
  phone_code_post_data: PhoneCodePostData,
  kv: &KvPool,
) -> ServiceResult<PhoneCodeResponseData> {
  // verify code
  // get random code
  let cfg = Config::get();
  let code = if cfg.env == ENV::Dev {
    "123456".to_string()
  } else {
    get_randome_code()
  };

  // add to kv
  let temp_key = get_phone_code_temp_key(
    &phone_code_post_data.phone_country_code,
    &phone_code_post_data.phone_number,
  );
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
      &req_meta.locale,
      Error::Other(format!("ttl: {}, body: {:?}", ttl, &phone_code_post_data)),
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
  let phone_text =
    &I18N
      .read()
      .unwrap()
      .with_args("phone-verify-code-template", &req_meta.locale, args);
  info!("Phone text: {}", phone_text);
  // todo send sms
  // todo add auths table
  Ok(PhoneCodeResponseData {
    meta: PhoneCodeMeta {
      length: code.len(),
      expires_in_minutes: cfg.auth.phone_code_verification_expires_in_minutes,
    },
  })
}
