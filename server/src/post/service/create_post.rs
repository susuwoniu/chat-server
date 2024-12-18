use crate::{
    account::{
        model::UpdateAccountParam,
        service::{get_account::get_full_account, update_account::update_account},
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::{config::get_random_background_color, Config},
    middleware::{Auth, Locale},
    post::{
        model::{CreatePostParam, DbPost, Post, UpdatePostTemplateParam, Visibility},
        service::{
            get_post::format_post, get_post_template::get_post_template,
            update_post_template::update_post_template,
        },
        util,
    },
    types::{DataWithMeta, FieldAction, Gender, ServiceResult},
    util::{
        date::{naive_to_beijing, naive_to_utc},
        id::next_id,
    },
};
use sonyflake::Sonyflake;

use chrono::{DateTime, Duration, Utc};
use ipnetwork17::IpNetwork;
use serde::{Deserialize, Serialize};
use sqlx::query_as;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NextPostMeta {
    pub next_post_not_before: DateTime<Utc>,
}
pub async fn create_post(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: CreatePostParam,
    auth: Auth,
    ip: IpNetwork,
    sf: &mut Sonyflake,
) -> ServiceResult<DataWithMeta<Post, NextPostMeta>> {
    let CreatePostParam {
        content,
        background_color,
        color,
        post_template_id,
        visibility,
        target_gender,
        latitude,
        longitude,
    } = param;
    // add post template
    let id = next_id(sf);
    let now = Utc::now().naive_utc();
    let cfg = Config::global();

    util::is_post_content_valid(locale, &content)?;
    let final_visibility = util::get_post_content_visibility(&content, visibility);
    let author = get_full_account(locale, pool, auth.account_id).await?;
    if author.suspended {
        return Err(ServiceError::account_suspended(
            locale,
            author.suspended_reason.clone(),
            author.suspended_until.clone(),
            Error::Other(format!("account {} suspened.", author.id)),
        ));
    }

    if author.next_post_not_before - now > Duration::minutes(5) {
        return Err(ServiceError::account_post_not_before(
            locale,
            naive_to_beijing(author.next_post_not_before.clone()),
            Error::Default,
        ));
    }
    // get post template info
    let post_template = get_post_template(locale, pool, post_template_id).await?;
    // get account

    let mut final_background_color = get_random_background_color();
    if let Some(background_color) = background_color {
        final_background_color = background_color;
    }

    let mut final_color: i64 = 4294967295;
    if let Some(color) = color {
        final_color = color;
    }

    let post = query_as!(DbPost,
    r#"
INSERT INTO posts (id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender,target_gender,visibility,approved,approved_at,approved_by,birthday,color,post_template_title,geom)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,
    CASE WHEN ($19::float8 is null or $20::float8 is null) THEN null ELSE ST_SetSRID(ST_Point($19,$20),4326) END)
RETURNING id,time_cursor_change_count,content,background_color,account_id,updated_at,post_template_title,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count,color,null::float8 as distance,favorite_count
"#,
    id,
    content,
    final_background_color as i64,
    auth.account_id,
    now,
    post_template_id,
    auth.client_id,
    id,
    ip,
    author.gender as Gender,
    target_gender as Option<Gender>,
    final_visibility as Visibility,
    true,
    now,
    auth.account_id,
    author.birthday,
    final_color,
    post_template.title,
    longitude,
    latitude
  )
  .fetch_one(pool)
  .await?;
    // update account post template count
    let account = update_account(
        locale,
        pool,
        kv,
        UpdateAccountParam {
            account_id: Some(auth.account_id),
            last_post_created_at: Some(now),
            post_count_action: Some(FieldAction::IncreaseOne),
            ..Default::default()
        },
        &auth,
        true,
    )
    .await?;
    update_post_template(
        locale,
        pool,
        post_template_id,
        UpdatePostTemplateParam {
            used_count_action: Some(FieldAction::IncreaseOne),
            ..Default::default()
        },
        auth.clone(),
        true,
    )
    .await?;
    let min_duration_between_posts_in_minutes = cfg.post.min_duration_between_posts_in_minutes;
    let vip_min_duration_between_posts_in_minutes =
        cfg.post.vip_min_duration_between_posts_in_minutes;
    let next_post_not_before;

    if account.vip || account.admin || account.admin {
        next_post_not_before = now + Duration::minutes(vip_min_duration_between_posts_in_minutes);
    } else {
        next_post_not_before = now + Duration::minutes(min_duration_between_posts_in_minutes);
    }

    return Ok(DataWithMeta {
        data: format_post(post, account.into(), Some(auth.clone()), Some(false)),
        meta: NextPostMeta {
            next_post_not_before: naive_to_utc(next_post_not_before),
        },
    });
}
