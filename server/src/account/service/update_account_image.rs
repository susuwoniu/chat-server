use crate::{
    account::{
        model::{
            DbProfileImage, FullAccount, UpdateAccountImageParam, UpdateAccountImagesParam,
            UpdateAccountParam,
        },
        service::update_account::update_account,
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::Config,
    middleware::{Auth, Locale},
    types::{Image, ServiceResult},
    util::{id::next_id, image::format_image as format_image_util},
};
use sonyflake::Sonyflake;

use chrono::{NaiveDateTime, Utc};
use sqlx::{query, query_as};
fn format_image(image: DbProfileImage) -> Image {
    return format_image_util(
        image.url,
        image.width,
        image.height,
        image.size,
        image.mime_type,
    );
}
pub async fn get_profile_images(pool: &Pool, account_id: i64) -> ServiceResult<Vec<Image>> {
    let images = query_as!(
        DbProfileImage,
        r#"
      select id,account_id,_order,url,size,height,width,mime_type,updated_at from account_images
      where account_id = $1 and deleted=false order by _order asc
  "#,
        account_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(images.into_iter().map(format_image).collect())
}
async fn update_account_image(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    auth: &Auth,
) -> ServiceResult<FullAccount> {
    // first get all images
    let images = get_profile_images(pool, auth.account_id).await?;
    update_account(
        locale,
        pool,
        kv,
        UpdateAccountParam {
            account_id: Some(auth.account_id),
            profile_images: Some(images),
            ..Default::default()
        },
        auth,
        true,
    )
    .await
}
pub async fn put_profile_images(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: UpdateAccountImagesParam,
    sf: &mut Sonyflake,
    auth: &Auth,
) -> ServiceResult<FullAccount> {
    let now = Utc::now();
    let account_id = auth.account_id;
    // 事务
    let mut tx = pool.begin().await?;
    let _ = query!(
        r#"
        UPDATE account_images
        SET deleted=true,
        updated_at=$2,
        deleted_at=$2
        where account_id = $1 and deleted=false
        "#,
        account_id,
        now.naive_utc(),
    )
    .execute(&mut tx)
    .await;
    let images = param.images;
    // insert all
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<NaiveDateTime> = Vec::new();
    let mut v3: Vec<i64> = Vec::new();
    let mut v4: Vec<i16> = Vec::new();
    let mut v5: Vec<String> = Vec::new();
    let mut v6: Vec<f64> = Vec::new();
    let mut v7: Vec<f64> = Vec::new();
    let mut v8: Vec<i64> = Vec::new();
    let mut v9: Vec<String> = Vec::new();
    let id = next_id(sf);
    let mut index = 0;
    images.into_iter().for_each(|row| {
        let final_id = id + index;
        v1.push(final_id);
        index = index + 1;
        v2.push(now.naive_utc());
        v3.push(account_id);
        v4.push(row.order);
        v5.push(row.url.clone());
        v6.push(row.width);
        v7.push(row.height);
        v8.push(row.size);
        v9.push(row.mime_type.clone());
    });

    sqlx::query(
    r#"
    INSERT into account_images 
    (id, updated_at, account_id, _order, url, width, height, size, mime_type) SELECT * FROM UNNEST ($1,$2,$3,$4,$5,$6,$7,$8,$9)
"#
  ).bind(&v1).bind(&v2).bind(&v3).bind(&v4).bind(&v5).bind(&v6).bind(&v7).bind(&v8).bind(&v9)
  .execute(&mut tx)
  .await?;
    tx.commit().await?;

    update_account_image(locale, pool, kv, auth).await
}

pub async fn insert_or_update_profile_image(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    auth: &Auth,
    param: UpdateAccountImageParam,
    sf: &mut Sonyflake,
) -> ServiceResult<FullAccount> {
    let now = Utc::now();
    let id = next_id(sf);
    let UpdateAccountImageParam {
        order,
        url,
        width,
        height,
        size,
        mime_type,
    } = param;
    let cfg = Config::global();
    if order >= cfg.account.max_profile_images {
        return Err(ServiceError::bad_request(
            locale,
            "reach_account_images_limit",
            Error::Other(format!("order: {}", order)),
        ));
    }
    query!(
        r#"
    INSERT into account_images 
    (id, updated_at, account_id, _order, url, width, height, size, mime_type)
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
    ON CONFLICT (account_id, _order,deleted_at)  DO UPDATE SET 
    updated_at=$2,
    url=$5,
    width =$6,
    height= $7,
    size=$8,
    mime_type=$9
"#,
        id,
        now.naive_utc(),
        auth.account_id,
        order,
        url,
        width,
        height,
        size,
        mime_type,
    )
    .execute(pool)
    .await?;

    update_account_image(locale, pool, kv, auth).await
}

pub async fn update_profile_image(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    auth: &Auth,
    param: UpdateAccountImageParam,
) -> ServiceResult<FullAccount> {
    let now = Utc::now();
    let cfg = Config::global();
    let account_id = auth.account_id;
    let UpdateAccountImageParam {
        order,
        url,
        width,
        height,
        size,
        mime_type,
    } = param;
    if order >= cfg.account.max_profile_images {
        return Err(ServiceError::bad_request(
            locale,
            "reach_account_images_limit",
            Error::Other(format!("order: {}", order)),
        ));
    }
    query!(
        r#"
    UPDATE account_images
    SET 
    updated_at=$3,
    url=COALESCE($4,url),
    _order =$2,
    width = COALESCE($5,width),
    height= COALESCE($6,height),
    size= COALESCE($7,size),
    mime_type = COALESCE($8,mime_type)
    where account_id = $1 and _order = $2 and deleted=false
    RETURNING id
"#,
        account_id,
        order,
        now.naive_utc(),
        url,
        width,
        height,
        size,
        mime_type,
    )
    .fetch_one(pool)
    .await?;

    update_account_image(locale, pool, kv, auth).await
}

pub async fn delete_profile_image(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    auth: &Auth,
    order: i16,
) -> ServiceResult<FullAccount> {
    let account_id = auth.account_id;
    let _ = query!(
        r#"
    UPDATE account_images
    set deleted=true,deleted_at=now()
    where account_id = $1 and _order = $2 and deleted=false
"#,
        account_id,
        order,
    )
    .execute(pool)
    .await;

    update_account_image(locale, pool, kv, auth).await
}
