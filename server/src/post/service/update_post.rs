use crate::{
    account::{
        model::UpdateAccountParam,
        service::{get_account::get_full_account, update_account::update_account},
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    post::{
        model::{DbPost, Post, PostFilter, UpdatePostParam, UpdatePostTemplateParam, Visibility},
        service::{
            get_post::{format_post, get_posts},
            update_post_template::update_post_template,
        },
    },
    types::{DataWithMeta, FieldAction, Gender, ServiceResult},
    util::{
        date::{naive_to_beijing, naive_to_utc},
        id::next_id,
    },
};
use serde::{Deserialize, Serialize};
use sonyflake::Sonyflake;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use sqlx::{query, query_as};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NextPostMeta {
    pub next_post_not_before: DateTime<Utc>,
}
pub async fn update_post(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    id: i64,
    param: UpdatePostParam,
    auth: Auth,
    sf: &mut Sonyflake,
) -> ServiceResult<DataWithMeta<Post, NextPostMeta>> {
    let cfg = Config::global();

    let UpdatePostParam {
        promote,
        viewed_count_action,
        skipped_count_action,
        approved,
        featured,
        visibility,
        deleted,
        replied_count_action,
        favorite_count_action,
    } = param;
    // get post
    let posts = get_posts(
        locale,
        pool,
        PostFilter {
            id: Some(id),
            ..Default::default()
        },
        Some(auth.clone()),
        true,
    )
    .await?;
    if &posts.data.len() == &0 {
        return Err(ServiceError::record_not_exist(
            &locale,
            "can_not_found_post",
            Error::Default,
        ));
    }
    let current = posts.data[0].clone();
    let post_author = current.author;
    let now = Utc::now().naive_utc();
    if post_author.suspended {
        return Err(ServiceError::account_suspended(
            locale,
            post_author.suspended_reason.clone(),
            post_author.suspended_until.clone(),
            Error::Other(format!("account {} suspened.", post_author.id)),
        ));
    }

    let mut featured_edit_value = None;
    let mut approved_edit_value = None;

    if auth.admin || auth.moderator {
        if let Some(featured_value) = featured {
            featured_edit_value = Some(featured_value);
        }
        if let Some(approved_value) = approved {
            approved_edit_value = Some(approved_value);
        }
    } else {
        // if self
        if auth.account_id != current.account_id {
            // 非admin/moderator/self only can update view_count, skipped_count, replied_count,
            if !(viewed_count_action.is_some()
                || skipped_count_action.is_some()
                || replied_count_action.is_some())
            {
                return Err(ServiceError::permission_limit(
                    locale,
                    "only_admin_or_moderator_or_self_can_update_post",
                    Error::Default,
                ));
            }
        }
    }

    let mut deleted_edit_value = None;
    if let Some(deleted_value) = deleted {
        if deleted_value {
            deleted_edit_value = Some(deleted_value);
        }
    }

    let mut featured_at = None;
    let mut featured_by = None;

    if featured_edit_value.is_some() {
        featured_at = Some(now);
        featured_by = Some(auth.account_id);
    }
    let mut approved_at = None;
    let mut approved_by = None;
    if approved_edit_value.is_some() {
        approved_at = Some(now);
        approved_by = Some(auth.account_id);
    }
    let mut deleted_at = None;
    let mut deleted_by = None;
    if deleted_edit_value.is_some() {
        deleted_at = Some(now);
        deleted_by = Some(auth.account_id);
    }
    // 修改view count // todo 修改view表
    let mut viewed_count_change_value = None;
    if let Some(viewed_count_action) = viewed_count_action {
        if current.account_id != auth.account_id {
            match viewed_count_action {
                FieldAction::IncreaseOne => {
                    // add to view table
                    // check user settings
                    if post_author.show_viewed_action {
                        // add to view table
                        let view_id = next_id(sf);
                        let view_insert_result = query!(
                            r#"INSERT INTO post_view 
          (id,viewed_by,post_id,post_account_id,updated_at)
          VALUES 
          ($1,$2,$3,$4,$5)
        "#,
                            view_id,
                            auth.account_id,
                            id,
                            current.account_id,
                            now
                        )
                        .execute(pool)
                        .await;
                        if view_insert_result.is_ok() {
                            // update post view count
                            viewed_count_change_value = Some(1);
                        }
                    }
                }
                FieldAction::DecreaseOne => {
                    // 暂时不支持减操作
                    viewed_count_change_value = None;
                }
            }
        }
    }
    //  修改skip count
    let mut skipped_count_change_value = None;
    if let Some(skipped_count_action) = skipped_count_action {
        if current.account_id != auth.account_id {
            match skipped_count_action {
                FieldAction::IncreaseOne => {
                    // Add to post_skip table
                    // TODO 优化，这一步可以延迟写入
                    let next_id = next_id(sf);
                    let insert_result = query!(
                        r#"INSERT INTO post_skip 
        (id,skipped_by,post_id,post_account_id,updated_at)
        VALUES 
        ($1,$2,$3,$4,$5)
      "#,
                        next_id,
                        auth.account_id,
                        id,
                        current.account_id,
                        now
                    )
                    .execute(pool)
                    .await;
                    if insert_result.is_ok() {
                        // update post view count
                        skipped_count_change_value = Some(1);
                    }
                }
                FieldAction::DecreaseOne => {
                    // 暂时不支持减操作
                    skipped_count_change_value = None;
                }
            }
        }
    }

    //  修改reply count
    let mut replied_count_change_value = None;
    if let Some(replied_count_action) = replied_count_action {
        if current.account_id != auth.account_id {
            match replied_count_action {
                FieldAction::IncreaseOne => {
                    let next_id = next_id(sf);
                    let insert_result = query!(
                        r#"INSERT INTO post_reply 
        (id,replied_by,post_id,post_account_id,updated_at)
        VALUES 
        ($1,$2,$3,$4,$5)
      "#,
                        next_id,
                        auth.account_id,
                        id,
                        current.account_id,
                        now
                    )
                    .execute(pool)
                    .await;
                    if insert_result.is_ok() {
                        // update post view count
                        replied_count_change_value = Some(1);
                    }
                }
                FieldAction::DecreaseOne => {
                    // 暂时不支持减操作
                    replied_count_change_value = None;
                }
            }
        }
    }
    //  修改favorite count
    let mut favorite_count_change_value = None;
    if let Some(favorite_count_action) = favorite_count_action {
        match favorite_count_action {
            FieldAction::IncreaseOne => {
                let next_id = next_id(sf);
                let query_result = query!(
                    r#"INSERT INTO post_favorites 
                        (id,account_id,post_id,post_account_id,updated_at)
                        VALUES ($1,$2,$3,$4,$5)
                        "#,
                    next_id,
                    auth.account_id,
                    id,
                    current.account_id,
                    now
                )
                .execute(pool)
                .await;
                if query_result.is_err() {
                    tracing::debug!(
                        "Duduplicate favorite from {} to post {}",
                        auth.account_id,
                        id
                    );
                    return Err(ServiceError::bad_request(
                        locale,
                        "duduplicated_favorite_action",
                        Error::Default,
                    ));
                } else {
                    favorite_count_change_value = Some(1);
                }
            }
            FieldAction::DecreaseOne => {
                let query_result = query!(
                    r#"
          DELETE from post_favorites where account_id=$1 and post_id=$2
          "#,
                    auth.account_id,
                    id,
                )
                .execute(pool)
                .await;

                if query_result.is_err() {
                    tracing::debug!(
                        "Duduplicate delete favorite from {} to post {}",
                        auth.account_id,
                        id,
                    );
                    return Err(ServiceError::bad_request(
                        locale,
                        "duduplicated_like_action",
                        Error::Default,
                    ));
                } else {
                    let query_result_parsed = query_result.unwrap();
                    if query_result_parsed.rows_affected() > 0 {
                        favorite_count_change_value = Some(-1);
                    } else {
                        // failed
                        tracing::debug!(
                            "not found favorite relation from {} to post {}",
                            auth.account_id,
                            id
                        );
                        return Err(ServiceError::bad_request(
                            locale,
                            "must_favorite_first",
                            Error::Default,
                        ));
                    }
                }
            }
        }
    }

    // 修改time cursor // 判断权限
    let mut time_cursor = None;
    let mut time_cursor_change_count = None;
    let vip_min_duration_between_posts_in_minutes =
        cfg.post.vip_min_duration_between_posts_in_minutes;
    if let Some(promote) = promote {
        if promote {
            if auth.admin || auth.moderator || auth.vip {
                // check time cursor change count

                if !current.is_can_promote {
                    return Err(ServiceError::reach_max_change_limit(
                        locale,
                        "max_time_cursor_change_count_reached",
                        "time-cursor",
                        None,
                        Error::Other(format!(
                            "post {} time_cursor reach max change limit",
                            current.id
                        )),
                    ));
                }

                // check laste post time

                if post_author.next_post_not_before - now > Duration::minutes(5) {
                    return Err(ServiceError::account_post_not_before(
                        locale,
                        naive_to_beijing(post_author.next_post_not_before.clone()),
                        Error::Default,
                    ));
                }

                time_cursor = Some(next_id(sf));
                time_cursor_change_count = Some(1);
                update_account(
                    locale,
                    pool,
                    kv,
                    UpdateAccountParam {
                        account_id: Some(auth.account_id),
                        last_post_created_at: Some(now),
                        ..Default::default()
                    },
                    &auth,
                    true,
                )
                .await?;
            } else {
                // 没权限
                return Err(ServiceError::permission_limit(
                    locale,
                    "only_admin_or_moderator_or_vip_can_update_post_time_cursor",
                    Error::Default,
                ));
            }
        }
    }

    let row =  query_as!(DbPost,
    r#"
UPDATE posts set 
featured = COALESCE($2,featured), 
featured_at = COALESCE($3,featured_at), 
featured_by = COALESCE($4,featured_by),
updated_at = $5, 
viewed_count=CASE WHEN $6::bigint is null THEN viewed_count ELSE viewed_count+$6::bigint END,
skipped_count=CASE WHEN $7::bigint is null THEN skipped_count ELSE skipped_count+$7::bigint END,
time_cursor=COALESCE($8,time_cursor),
approved = COALESCE($9,approved), 
approved_at = COALESCE($10,approved_at), 
approved_by = COALESCE($11,approved_by),
visibility = COALESCE($12,visibility),
deleted = COALESCE($13,deleted), 
deleted_at = COALESCE($14,deleted_at), 
deleted_by = COALESCE($15,deleted_by),
replied_count=CASE WHEN $16::bigint is null THEN replied_count ELSE replied_count+$16::bigint END,
time_cursor_change_count=CASE WHEN $17::int is null THEN time_cursor_change_count ELSE time_cursor_change_count+$17::int END,
favorite_count=CASE WHEN $18::bigint is null THEN favorite_count ELSE favorite_count+$18::bigint END
WHERE id = $1 and deleted = false
RETURNING id,time_cursor_change_count,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,post_template_title,replied_count,color,null::float8 as distance,favorite_count
"#,    id,
    featured_edit_value,
    featured_at,
    featured_by,
    now,
    viewed_count_change_value,
    skipped_count_change_value,
    time_cursor,
    approved_edit_value,
    approved_at,
    approved_by,
    visibility.clone() as Option<Visibility>,
    deleted_edit_value,
    deleted_at,
    deleted_by,
    replied_count_change_value,
    time_cursor_change_count as Option<i32>,
    favorite_count_change_value

  )
  .fetch_one(pool)
  .await?;
    // 更新account， 更新post_template
    if deleted_edit_value.is_some() || visibility.is_some() {
        if deleted_edit_value.is_some() {
            update_account(
                locale,
                pool,
                kv,
                UpdateAccountParam {
                    account_id: Some(auth.account_id),
                    post_count_action: Some(FieldAction::DecreaseOne),
                    ..Default::default()
                },
                &auth,
                true,
            )
            .await?;
        }
        if deleted_edit_value.is_some() || visibility == Some(Visibility::Private) {
            // todo used count
            update_post_template(
                locale,
                pool,
                current.post_template_id,
                UpdatePostTemplateParam {
                    used_count_action: Some(FieldAction::DecreaseOne),
                    ..Default::default()
                },
                auth.clone(),
                true,
            )
            .await?;
        }
    }
    let account = get_full_account(locale, pool, row.account_id).await?;
    let next_post_not_before = account
        .last_post_created_at
        .unwrap_or(NaiveDateTime::from_timestamp(0, 0))
        + Duration::minutes(vip_min_duration_between_posts_in_minutes);
    return Ok(DataWithMeta {
        data: format_post(row, account.into(), Some(auth)),
        meta: NextPostMeta {
            next_post_not_before: naive_to_utc(next_post_not_before),
        },
    });
}
