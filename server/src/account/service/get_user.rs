use crate::account::model::SlimAccount;
use crate::config::Config;
use crate::error::ServiceResult;
use crate::i18n::I18n;
use crate::middleware::req_meta::ReqMeta;
use crate::types::KvPool;
use crate::util::id::next_id;

pub async fn get_user(
  _req_meta: ReqMeta,
  _kv: &KvPool,
  _i18n: &I18n,
  _config: &Config,
) -> ServiceResult<SlimAccount> {
  // verify code
  let id = next_id();

  // todo add auths table
  Ok(SlimAccount { id })
}
