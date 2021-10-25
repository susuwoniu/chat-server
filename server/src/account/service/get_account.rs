use crate::{
  account::model::{Account, Gender, SlimAccount},
  alias::Pool,
  error::{Error, ServiceError, ServiceResult},
  global::Config,
  middleware::Locale,
};
use chrono::offset::FixedOffset;
use chrono::Datelike;
use chrono::{Date, Utc};
use sqlx::query;
pub async fn get_slim_account(
  pool: &Pool,
  account_id: &i64,
  locale: &Locale,
) -> ServiceResult<SlimAccount> {
  return Ok(get_account(pool, account_id, locale).await?.into());
}
pub async fn get_account(pool: &Pool, account_id: &i64, locale: &Locale) -> ServiceResult<Account> {
  let account_row = query!(
    r#"
      select id,name,bio,gender as "gender:Gender",admin,moderator,vip,posts_count,likes_count,show_age,show_distance,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,profile_images,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id from accounts where id = $1 and deleted=false
"#,
    account_id
  )
  .fetch_optional(pool)
  .await?;
  let cfg = Config::global();
  if let Some(account) = account_row {
    // todo add auths table
    // get age
    //
    let now = Utc::now();

    let current_utc_year = now.year();
    let mut age = None;
    if let Some(raw_birthday) = account.birthday {
      let mut tz = FixedOffset::east(cfg.default_timezone_offset_in_seconds);
      if let Some(account_tz) = account.timezone_in_seconds {
        tz = FixedOffset::east(account_tz);
      }
      let birthday_with_tz: Date<FixedOffset> = Date::from_utc(raw_birthday, tz);
      let birthday_utc = birthday_with_tz.naive_utc();
      let birthday_utc_year = birthday_utc.year();
      age = Some(current_utc_year - birthday_utc_year);
    }

    Ok(
      Account {
        id: account.id,
        name: account.name,
        bio: account.bio,
        gender: account.gender,
        admin: account.admin,
        moderator: account.moderator,
        vip: account.vip,
        posts_count: account.posts_count,
        likes_count: account.likes_count,
        show_age: account.show_age,
        show_distance: account.show_distance,
        suspended: account.suspended,
        suspended_at: account.suspended_at,
        suspended_until: account.suspended_until,
        suspended_reason: account.suspended_reason,
        age: age,
        birthday: account.birthday,
        timezone_in_seconds: account.timezone_in_seconds,
        phone_country_code: account.phone_country_code,
        phone_number: account.phone_number,
        location: account.location,
        country_id: account.country_id,
        state_id: account.state_id,
        city_id: account.city_id,
        avatar: account.avatar,
        profile_images: account.profile_images,
        avatar_updated_at: account.avatar_updated_at,
        created_at: account.created_at,
        updated_at: account.updated_at,
        approved: account.approved,
        approved_at: account.approved_at,
        invite_id: account.invite_id,
      }
      .into(),
    )
  } else {
    return Err(ServiceError::account_not_exist(
      locale,
      Error::Other(format!("Can not found account_id: {} at db", account_id)),
    ));
  }
}
