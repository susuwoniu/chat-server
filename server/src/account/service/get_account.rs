use crate::{
  account::{
    model::{Account, Gender, SlimAccount},
    service::update_account_image::get_profile_images,
  },
  alias::Pool,
  error::{Error, ServiceError},
  global::Config,
  middleware::Locale,
  types::{Action, ActionType, ServiceResult},
};
use chrono::offset::FixedOffset;
use chrono::Datelike;
use chrono::{Date, Utc};
use sqlx::query;
pub async fn get_slim_account(
  locale: &Locale,
  pool: &Pool,
  account_id: &i64,
) -> ServiceResult<SlimAccount> {
  return Ok(get_account(locale, pool, account_id).await?.into());
}
pub async fn get_account(locale: &Locale, pool: &Pool, account_id: &i64) -> ServiceResult<Account> {
  let account_row = query!(
    r#"
      select id,name,bio,gender as "gender:Gender",admin,moderator,vip,posts_count,likes_count,show_age,show_distance,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id,name_change_count,bio_change_count,gender_change_count,birthday_change_count,phone_change_count,skip_optional_info,profile_image_change_count from accounts where id = $1 and deleted=false
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
    // check

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
    // todo 并行
    let profile_images = get_profile_images(pool, &account.id).await?;
    let mut actions: Vec<Action> = Vec::new();

    // required info
    if account.birthday_change_count == 0 {
      actions.push(Action {
        _type: ActionType::AddAccountBirthday,
        required: true,
      });
    }
    if account.gender_change_count == 0 {
      actions.push(Action {
        _type: ActionType::AddAccountGender,
        required: true,
      });
    }
    // optional info

    if account.skip_optional_info == false {
      if account.name_change_count == 0 {
        actions.push(Action {
          _type: ActionType::AddAccountName,
          required: false,
        });
      }
      if account.bio_change_count == 0 {
        actions.push(Action {
          _type: ActionType::AddAccountBio,
          required: false,
        });
      }
      if account.profile_image_change_count == 0 {
        actions.push(Action {
          _type: ActionType::AddAccountProfileImage,
          required: false,
        });
      }
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
        profile_images: profile_images,
        city_id: account.city_id,
        avatar: account.avatar,
        avatar_updated_at: account.avatar_updated_at,
        created_at: account.created_at,
        updated_at: account.updated_at,
        approved: account.approved,
        approved_at: account.approved_at,
        invite_id: account.invite_id,
        actions,
        name_change_count: account.name_change_count,
        bio_change_count: account.bio_change_count,
        birthday_change_count: account.birthday_change_count,
        phone_change_count: account.phone_change_count,
        gender_change_count: account.gender_change_count,
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
