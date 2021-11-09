use crate::{
  account::{model::UpdateAccountParam, service::get_account::get_account},
  alias::Pool,
  error::{Error, ServiceError},
  global::Config,
  middleware::{Auth, Locale},
  types::ServiceResult,
};

use chrono::Utc;
use sqlx::query;
pub async fn update_account(
  locale: &Locale,
  pool: &Pool,
  account_id: &i64,
  param: UpdateAccountParam,
  auth: &Auth,
) -> ServiceResult<()> {
  // first get account
  let is_admin = auth.admin;
  let is_vip = auth.vip;
  let is_moderator = auth.moderator;
  let now = Utc::now();
  let cfg = Config::global();
  let trace_info = format!("param:{:?}", &param);
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
    approved,
    invite_id,
  } = param;
  // check permiss

  // only admin fields

  if !is_admin && (admin.is_some() || moderator.is_some()) {
    return Err(ServiceError::permission_limit(
      locale,
      "no_permission_to_modify_admin_or_moderator",
      Error::Other(trace_info),
    ));
  }
  // only admin or moderator
  if (!is_admin || !is_moderator) && suspended.is_some()
    || suspended_at.is_some()
    || suspended_until.is_some()
    || suspended_reason.is_some()
  {
    return Err(ServiceError::permission_limit(
      locale,
      "no_permission_to_modify_suspended",
      Error::Other(trace_info),
    ));
  }

  // only vip

  if (!is_vip || !is_admin) && show_age.is_some() || show_distance.is_some() {
    return Err(ServiceError::permission_limit(
      locale,
      "no_permission_to_modify_show_age_or_show_distance",
      Error::Other(trace_info),
    ));
  }

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

  let mut approved_at = None;
  if approved.is_some() {
    approved_at = Some(now.naive_utc());
  }
  let mut birthday_change_count = None;
  if let Some(birthday) = birthday {
    // 修改次数限制
    if account.birthday_change_count >= 1 {
      // 不能再改
      return Err(ServiceError::reach_max_change_limit(
        locale,
        "birthday_reach_max_change_limit",
        "birthday",
        None,
        Error::Other(format!(
          "account {} birthday reach max change limit",
          account.id
        )),
      ));
    }
    // birthday must > 18
    let duration = now.date().naive_utc() - birthday;
    let min_days = cfg.account.min_age * 365;
    let is_valid = duration.num_days() > min_days as i64;
    if !is_valid {
      return Err(ServiceError::account_age_invalid(
        locale,
        Error::Other(format!(
          "account {} age invalid, birthday: {}.",
          account.id, birthday
        )),
      ));
    }

    if account.birthday.is_none() || Some(birthday) != account.birthday {
      birthday_change_count = Some(account.birthday_change_count + 1);
    }
  }
  let mut name_change_count = None;
  if let Some(name) = name.clone() {
    if name != account.name {
      name_change_count = Some(account.name_change_count + 1);
    }
  }
  let mut bio_change_count = None;
  if let Some(bio) = bio.clone() {
    if bio != account.bio {
      bio_change_count = Some(account.bio_change_count + 1);
    }
  }
  let mut gender_change_count = None;
  if let Some(gender) = gender.clone() {
    if account.gender_change_count >= 1 {
      // 不能再改
      return Err(ServiceError::reach_max_change_limit(
        locale,
        "gender_reach_max_change_limit",
        "gender",
        None,
        Error::Other(format!(
          "account {} gender reach max change limit",
          account.id
        )),
      ));
    }
    if gender != account.gender {
      gender_change_count = Some(account.gender_change_count + 1);
    }
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
    approved_at=COALESCE($25,approved_at),
    birthday_change_count=COALESCE($26,birthday_change_count),
    name_change_count=COALESCE($27,name_change_count),
    bio_change_count=COALESCE($28,bio_change_count),
    gender_change_count=COALESCE($29,bio_change_count)
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
    approved_at,
    birthday_change_count,
    name_change_count,
    bio_change_count,
    gender_change_count
  )
  .execute(pool)
  .await?;
  //

  Ok(())
}
