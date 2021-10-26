use crate::{
  account::{
    model::{Account, Gender, SuccessResponseData, UpdateAccountParam},
    service::get_account::get_account,
  },
  alias::Pool,
  error::{Error, ServiceError, ServiceResult},
  global::Config,
  middleware::Locale,
};
use chrono::offset::FixedOffset;
use chrono::Datelike;
use chrono::{Date, Utc};
use sqlx::query;
pub async fn update_account(
  locale: &Locale,
  pool: &Pool,
  account_id: &i64,
  param: UpdateAccountParam,
) -> ServiceResult<SuccessResponseData> {
  // first get account
  let now = Utc::now();
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

  let UpdateAccountParam {
    name,
    bio,
    gender,
    admin,
    moderator,
    vip,
    show_age,
    show_distance,
    suspended,
    suspended_at,
    suspended_until,
    suspended_reason,
    birthday,
    timezone_in_seconds,
    phone_country_code,
    phone_number,
    location,
    country_id,
    state_id,
    city_id,
    avatar,
    profile_images,
    approved,
    invite_id,
  } = param;
  let mut avatar_updated_at = None;
  if avatar.is_some() {
    avatar_updated_at = Some(now.naive_utc());
  }
  let mut approved_at = None;
  if approved.is_some() {
    approved_at = Some(now.naive_utc());
  }
  query!(
    r#"
    UPDATE accounts 
    SET 
    updated_at=$2,
    name=COALESCE($3,name),
    bio=COALESCE($4,bio),
    gender=COALESCE($5,gender),
    admin=COALESCE($6,admin),
    moderator=COALESCE($7,moderator),
    vip=COALESCE($8,vip),
    show_age=COALESCE($9,show_age),
    show_distance=COALESCE($10,show_distance),
    suspended=COALESCE($11,suspended),
    invite_id=COALESCE($12,invite_id),
    suspended_at=COALESCE($13,suspended_at),
    suspended_until=COALESCE($14,suspended_until),
    suspended_reason=COALESCE($15,suspended_reason),
    birthday=COALESCE($16,birthday),
    timezone_in_seconds=COALESCE($17,timezone_in_seconds),
    phone_country_code=COALESCE($18,phone_country_code),
    phone_number=COALESCE($19,phone_number),
    location=COALESCE($20,location),
    approved=COALESCE($21,approved),
    country_id=COALESCE($22,country_id),
    state_id=COALESCE($23,state_id),
    city_id=COALESCE($24,city_id),
    avatar=COALESCE($25,avatar),
    avatar_updated_at=COALESCE($26,avatar_updated_at),
    approved_at=COALESCE($27,approved_at),
    profile_images=CoALESCE($28,profile_images)
    where id = $1
"#,
    account_id,
    now.naive_utc(),
    name,
    bio,
    gender as _,
    admin,
    moderator,
    vip,
    show_age,
    show_distance,
    suspended,
    invite_id,
    suspended_at,
    suspended_until,
    suspended_reason,
    birthday,
    timezone_in_seconds,
    phone_country_code,
    phone_number,
    location,
    approved,
    country_id,
    state_id,
    city_id,
    avatar,
    avatar_updated_at,
    approved_at,
    profile_images
  )
  .execute(pool)
  .await?;
  //

  Ok(SuccessResponseData::default())
}
