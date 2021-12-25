use crate::{
    account::{
        model::{DbAccount, FullAccount, UpdateAccountParam, UpdateOtherAccountParam},
        service::get_account::{format_account, get_full_account},
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::Config,
    im::{model::ImUpdateAccountParam, service::update_im_account::update_im_account},
    middleware::{Auth, Locale},
    notification::{
        model::{
            CreateNotificationParam, NotificationAction, NotificationActionData, NotificationType,
            ProfileLikeedActionData, ProfileViewedActionData,
        },
        service::create_notification::create_notification,
    },
    types::{FieldAction, Gender, JsonVersion, ServiceResult},
    util::id::next_id,
};

use chrono::Utc;
use sqlx::{query, query_as};
// 修改他人的账户
pub async fn update_other_account(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: UpdateOtherAccountParam,
    auth: Auth,
) -> ServiceResult<()> {
    let account_id = auth.account_id;
    let mut field_action = None;
    let now = Utc::now();
    let mut is_primary = false;
    let mut _type = NotificationType::ProfileViewed;
    let mut action = NotificationAction::ProfileViewed;
    let mut action_data = NotificationActionData::ProfileViewed(ProfileViewedActionData {
        version: JsonVersion::V1,
    });
    let UpdateOtherAccountParam {
        viewed_count_action,
        target_account_id,
        like_count_action,
    } = param;
    // 是否本人
    if target_account_id == account_id {
        return Ok(());
    }
    // 互斥，only one once.
    if viewed_count_action.is_some() && like_count_action.is_some() {
        return Err(ServiceError::bad_request(
            locale,
            "only_one_action_can_edited_once",
            Error::Default,
        ));
    }
    // 修改count
    let default_viewed_count = 1;
    if let Some(viewed_count_action) = viewed_count_action {
        match viewed_count_action {
            FieldAction::IncreaseOne => {
                // TODO
                let mut tx = pool.begin().await?;
                let id = next_id();
                query!(
                    r#"
      INSERT INTO account_view_records (id,viewed_by,target_account_id,updated_at,created_at)
      VALUES ($1,$2,$3,$4,$5)
      "#,
                    id,
                    account_id,
                    target_account_id,
                    now.naive_utc(),
                    now.naive_utc()
                )
                .execute(&mut tx)
                .await?;
                query!(
                    r#"
          INSERT into account_views as t 
          (id, updated_at,created_at,viewed_by, target_account_id,viewed_count)
          VALUES ($1,$2,$3,$4,$5,$6) 
          ON CONFLICT (viewed_by, target_account_id)  DO UPDATE SET 
          updated_at=$2,
          viewed_count=t.viewed_count+1
      "#,
                    id,
                    now.naive_utc(),
                    now.naive_utc(),
                    account_id,
                    target_account_id,
                    default_viewed_count
                )
                .execute(&mut tx)
                .await?;
                tx.commit().await?;
                field_action = Some(FieldAction::IncreaseOne);
            }
            FieldAction::DecreaseOne => {
                // viewed_count_action_value = None;
            }
        }
    }

    // if like_count_action exist
    let mut is_increase_likes_count_success = false;
    let mut is_delete_likes_success = false;
    if let Some(like_count_action) = like_count_action {
        _type = NotificationType::ProfileLiked;
        action = NotificationAction::ProfileLiked;
        action_data = NotificationActionData::ProfileLiked(ProfileLikeedActionData {
            version: JsonVersion::V1,
        });
        match like_count_action {
            FieldAction::IncreaseOne => {
                // TODO
                let id = next_id();

                let query_result = query!(
                    r#"
      INSERT INTO likes (id,account_id,target_account_id,updated_at,created_at)
      VALUES ($1,$2,$3,$4,$5)
      "#,
                    id,
                    account_id,
                    target_account_id,
                    now.naive_utc(),
                    now.naive_utc()
                )
                .execute(pool)
                .await;
                if query_result.is_err() {
                    tracing::warn!(
                        "Duduplicate like from {} to account {}",
                        account_id,
                        target_account_id
                    );
                } else {
                    is_primary = true;
                    is_increase_likes_count_success = true;
                    field_action = Some(FieldAction::IncreaseOne);
                }
            }
            FieldAction::DecreaseOne => {
                let query_result = query!(
                    r#"
      DELETE from likes where account_id=$1 and target_account_id=$2
      "#,
                    account_id,
                    target_account_id,
                )
                .execute(pool)
                .await;
                if query_result.is_err() {
                    tracing::warn!(
                        "Duduplicate like from {} to account {}",
                        account_id,
                        target_account_id
                    );
                } else {
                    field_action = Some(FieldAction::DecreaseOne);
                    is_delete_likes_success = true;
                }
            }
        }
    }

    // update account
    if is_increase_likes_count_success {
        //
        update_account(
            locale,
            pool,
            kv,
            UpdateAccountParam {
                account_id: Some(target_account_id),
                like_count_action: Some(FieldAction::IncreaseOne),
                ..Default::default()
            },
            &auth,
            true,
        )
        .await?;
    } else if is_delete_likes_success {
        // 删除点赞
        update_account(
            locale,
            pool,
            kv,
            UpdateAccountParam {
                account_id: Some(target_account_id),
                like_count_action: Some(FieldAction::DecreaseOne),
                ..Default::default()
            },
            &auth,
            true,
        )
        .await?;
    }
    if field_action.is_some() {
        // 创建通知
        let notification = CreateNotificationParam {
            content: "".to_string(),
            _type: _type,
            action: action,
            target_account_id,
            is_primary: is_primary,
            field_action: field_action.unwrap(),
            action_data: action_data,
        };
        // create notification
        create_notification(locale, pool, kv, notification, auth.clone()).await?;
    }

    return Ok(());
}
pub async fn update_account(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: UpdateAccountParam,
    auth: &Auth,
    is_internal: bool,
) -> ServiceResult<FullAccount> {
    // first get account
    let auth_account_id = auth.account_id;
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
        skip_optional_info,
        post_template_count_action,
        post_count_action,
        like_count_action,
        show_viewed_action,
        account_id: account_id_value,
    } = param;
    let account_id = account_id_value.unwrap_or(auth_account_id);

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
    if (!is_admin || !is_moderator)
        && (suspended.is_some()
            || suspended_at.is_some()
            || suspended_until.is_some()
            || suspended_reason.is_some())
    {
        return Err(ServiceError::permission_limit(
            locale,
            "no_permission_to_modify_suspended",
            Error::Other(trace_info),
        ));
    }
    // only internal
    if !is_internal
        && (post_count_action.is_some()
            || post_template_count_action.is_some()
            || like_count_action.is_some())
    {
        return Err(ServiceError::permission_limit(
            locale,
            "no_permission_to_modify_internal_only",
            Error::Other(trace_info),
        ));
    }

    // only vip

    if (!is_vip || !is_admin)
        && (show_age.is_some() || show_distance.is_some() || show_viewed_action.is_some())
    {
        return Err(ServiceError::permission_limit(
            locale,
            "no_permission_to_modify_vip_only_properties",
            Error::Other(trace_info),
        ));
    }
    // only these fields than others can change
    if auth_account_id != account_id
        && (name.is_some()
            || bio.is_some()
            || gender.is_some()
            || admin.is_some()
            || moderator.is_some()
            || vip.is_some()
            || show_age.is_some()
            || show_distance.is_some()
            || suspended.is_some()
            || suspended_at.is_some()
            || suspended_until.is_some()
            || suspended_reason.is_some()
            || birthday.is_some()
            || timezone_in_seconds.is_some()
            || phone_country_code.is_some()
            || phone_number.is_some()
            || location.is_some()
            || country_id.is_some()
            || state_id.is_some()
            || city_id.is_some()
            || approved.is_some()
            || invite_id.is_some()
            || skip_optional_info.is_some()
            || show_viewed_action.is_some())
    {
        return Err(ServiceError::permission_limit(
            locale,
            "no_permission_to_modify_self_only_properties",
            Error::Other(trace_info),
        ));
    }

    if account_id == auth_account_id && like_count_action.is_some() {
        return Err(ServiceError::permission_limit(
            locale,
            "no_permission_to_modify_other_only_properties",
            Error::Other(trace_info),
        ));
    }

    let account = get_full_account(locale, pool, account_id).await?;
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
        // TODO
        if account.birthday_change_count >= 100 {
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
            // update im name

            update_im_account(
                kv,
                ImUpdateAccountParam {
                    account_id,
                    name: Some(name),
                    ..Default::default()
                },
            )
            .await?;
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
        if account.gender_change_count >= 100 {
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

    let mut post_template_count_value = None;

    if let Some(post_template_count_action) = post_template_count_action {
        match post_template_count_action {
            FieldAction::IncreaseOne => {
                post_template_count_value = Some(account.post_template_count + 1);
            }
            FieldAction::DecreaseOne => {
                post_template_count_value = Some(account.post_template_count - 1);
            }
        }
    }
    let mut post_count_value = None;

    if let Some(post_count_action) = post_count_action {
        match post_count_action {
            FieldAction::IncreaseOne => {
                post_count_value = Some(account.post_count + 1);
            }
            FieldAction::DecreaseOne => {
                post_count_value = Some(account.post_count - 1);
            }
        }
    }

    let mut like_count_changed_value: Option<i32> = None;

    if let Some(like_count_action) = like_count_action {
        match like_count_action {
            FieldAction::IncreaseOne => {
                like_count_changed_value = Some(1);
            }
            FieldAction::DecreaseOne => {
                like_count_changed_value = Some(-1);
            }
        }
    }

    let account_row = query_as!(DbAccount,
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
    gender_change_count=COALESCE($29,gender_change_count),
    skip_optional_info=COALESCE($30,skip_optional_info),
    post_template_count=COALESCE($31,post_template_count),
    post_count=COALESCE($32,post_count), 
    like_count=CASE WHEN $33::bigint is null THEN like_count ELSE like_count+$33::bigint END,
    show_viewed_action=COALESCE($34,show_viewed_action)
    where id = $1
    RETURNING id,name,bio,gender as "gender:Gender",admin,moderator,vip,post_count,like_count,show_age,show_distance,suspended,suspended_at,suspended_until,suspended_reason,birthday,timezone_in_seconds,show_viewed_action,phone_country_code,phone_number,location,country_id,state_id,city_id,avatar,avatar_updated_at,created_at,updated_at,approved,approved_at,invite_id,name_change_count,bio_change_count,gender_change_count,birthday_change_count,phone_change_count,skip_optional_info,profile_image_change_count,post_template_count,profile_images
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
    gender_change_count,
    skip_optional_info,
    post_template_count_value,
    post_count_value,
    like_count_changed_value as Option<i32>,
    show_viewed_action
  )
  .fetch_one(pool)
  .await?;
    //

    return Ok(format_account(account_row));
}
