use crate::{
  account::{
    model::{IdentityType, SigninParam, SigninType, SignupData, SignupParam},
    service::signin::signin,
    util::{get_phone_code_temp_key, AuthData},
  },
  alias::{KvPool, Pool},
  error::{Error, ServiceError},
  global::{Config, I18n},
  middleware::Locale,
  types::ServiceResult,
  util::{id::next_id, string::get_random_letter},
};

use chrono::Utc;
use fluent_bundle::FluentArgs;
use sqlx::query;
pub async fn signup(locale: &Locale, pool: &Pool, param: SignupParam) -> ServiceResult<SignupData> {
  let account_id = next_id();
  let cfg = Config::global();
  let now = Utc::now();
  // get random name
  let mut args = FluentArgs::new();
  args.set("random", get_random_letter(4));
  let default_name = I18n::global().with_args("default-name", &locale, args);
  let mut tx = pool.begin().await?;
  let SignupParam {
    phone_country_code,
    phone_number,
    identity_type,
    identifier,
    timezone_in_seconds,
  } = param;

  if identity_type == IdentityType::Phone
    && (phone_country_code.is_none() || phone_number.is_none())
  {
    // must suppliy
    return Err(ServiceError::bad_request(
      locale,
      "phone_empty",
      Error::Default,
    ));
  }
  let mut approved_at = None;
  let approved = !cfg.invite_only;
  if approved {
    approved_at = Some(now.naive_utc());
  }
  // add acccount
  query!(
    r#"
INSERT INTO accounts (id,name,phone_country_code,phone_number,updated_at,timezone_in_seconds,approved,approved_at)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
"#,
    account_id,
    default_name,
    phone_country_code,
    phone_number,
    Utc::now().naive_utc(),
    timezone_in_seconds,
    approved,
    approved_at

  )
  .execute(&mut tx)
  .await?;
  let account_auth_id = next_id();

  // add account_auths
  // TODO source_from, sign_up_ip, invite_id
  query!(
    r#"
INSERT INTO account_auths (id,identity_type,identifier,account_id,updated_at)
VALUES ($1,'phone',$2,$3,$4)
"#,
    account_auth_id,
    identifier,
    account_id,
    now.naive_utc()
  )
  .execute(&mut tx)
  .await?;
  tx.commit().await?;
  Ok(SignupData {
    account_id,
    account_auth_id,
  })
}