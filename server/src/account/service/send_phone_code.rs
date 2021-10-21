use super::util::get_phone_code_temp_key;
use crate::account::model::{PhoneCodePostData, SlimAccount};
use crate::config::Config;
use crate::errors::ServiceResult;
use crate::i18n::I18n;
use crate::middleware::req_meta::ReqMeta;
use crate::types::KvPool;
use crate::util::id::next_id;
use deadpool_redis::redis::cmd;
use fluent::FluentArgs;
pub async fn send_phone_code(
  req_meta: ReqMeta,
  phone_code_post_data: PhoneCodePostData,
  kv: &KvPool,
  i18n: &I18n,
  config: &Config,
) -> ServiceResult<SlimAccount> {
  // verify code
  let id = next_id();
  // get random code
  // todo
  let code = "123456";

  // add to kv
  let temp_key = get_phone_code_temp_key(
    phone_code_post_data.phone_country_code,
    phone_code_post_data.phone_number,
  );
  let mut conn = kv.get().await?;
  cmd("SET")
    .arg(&[
      &temp_key,
      code,
      "EX",
      &(config.auth.phone_code_verification_expires_in_minutes * 60).to_string(),
    ])
    .query_async::<_, ()>(&mut conn)
    .await?;
  let mut args = FluentArgs::new();
  args.set("code", code);
  let phone_text = i18n.with_args("phone-verify-code-template", &req_meta.locale, args);
  info!("Phone text: {}", phone_text);

  // todo add auths table
  Ok(SlimAccount { id })
}
