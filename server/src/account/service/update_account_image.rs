use crate::{
    account::model::{
        DbProfileImage, DbProfileImagesJson, ProfileImage, Thumbtail, UpdateAccountImageParam,
        UpdateAccountImagesParam,
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::Config,
    middleware::Locale,
    types::{JsonVersion, ServiceResult},
    util::id::next_id,
};
use sonyflake::Sonyflake;

use chrono::{NaiveDateTime, Utc};
use serde_json::json;
use sqlx::{query, query_as};
fn format_image(image: DbProfileImage) -> ProfileImage {
    let thumbnail_default_width = 300;
    let thumbnail_mime_type = "image/webp";
    let thumbnail_default_height = image.height * thumbnail_default_width as f64 / image.width;
    let thumbtail_url = format!("{}/{}", image.url, "/thumbtail");
    return ProfileImage {
        id: image.id,
        account_id: image.account_id,
        url: image.url,
        width: image.width,
        height: image.height,
        order: image._order,
        size: image.size,
        mime_type: image.mime_type,
        updated_at: image.updated_at,
        thumbtail: Thumbtail {
            url: thumbtail_url,
            width: thumbnail_default_width as f64,
            height: thumbnail_default_height,
            mime_type: thumbnail_mime_type.to_string(),
        },
    };
}
pub async fn get_profile_images(pool: &Pool, account_id: i64) -> ServiceResult<Vec<ProfileImage>> {
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
    pool: &Pool,
    account_id: i64,
    is_update_avatar: bool,
    avatar: Option<ProfileImage>,
) -> ServiceResult<()> {
    let now = Utc::now();
    // first get all images
    let images = get_profile_images(pool, account_id).await?;
    let mut avatar_url: Option<String> = None;
    if let Some(image) = avatar {
        avatar_url = Some(format!("{}/{}", image.url, "avatar"));
    }

    query!(
        r#"
UPDATE accounts
SET 
updated_at=$2,
avatar_updated_at= case when $3=true then $2 else avatar_updated_at end,
avatar = case when $3=true then $4 else avatar end,
profile_image_change_count=profile_image_change_count+1,
profile_images = $5
where id = $1
"#,
        account_id,
        now.naive_utc(),
        is_update_avatar,
        avatar_url,
        json!(DbProfileImagesJson {
            images: images,
            version: JsonVersion::V1
        })
    )
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn put_profile_images(
    _: &Locale,
    pool: &Pool,
    account_id: &i64,
    param: UpdateAccountImagesParam,
    sf: &mut Sonyflake,
) -> ServiceResult<Vec<ProfileImage>> {
    let now = Utc::now();

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
    // todo avatar
    let mut db_images: Vec<ProfileImage> = Vec::new();
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
        v3.push(*account_id);
        v4.push(row.order);
        v5.push(row.url.clone());
        v6.push(row.width);
        v7.push(row.height);
        v8.push(row.size);
        v9.push(row.mime_type.clone());
        db_images.push(format_image(DbProfileImage {
            id: final_id,
            account_id: *account_id,
            _order: row.order,
            url: row.url,
            width: row.width,
            height: row.height,
            size: row.size,
            mime_type: row.mime_type,
            updated_at: now.naive_utc(),
        }))
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
    if db_images.len() > 0 {
        let image = db_images[0].clone();
        update_account_image(pool, *account_id, true, Some(image)).await?;
    } else {
        // remove avatar
        update_account_image(pool, *account_id, true, None).await?;
    }
    return Ok(db_images);
}

pub async fn insert_or_update_profile_image(
    locale: &Locale,
    pool: &Pool,
    _: &KvPool,
    account_id: &i64,
    param: UpdateAccountImageParam,
    sf: &mut Sonyflake,
) -> ServiceResult<ProfileImage> {
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
        account_id,
        order,
        url,
        width,
        height,
        size,
        mime_type,
    )
    .execute(pool)
    .await?;

    // update avatar
    let image = format_image(DbProfileImage {
        id,
        account_id: *account_id,
        _order: order,
        url,
        updated_at: now.naive_utc(),
        width,
        height,
        size,
        mime_type,
    });
    if order == 0 {
        update_account_image(pool, *account_id, true, Some(image.clone())).await?;
    } else {
        update_account_image(pool, *account_id, false, None).await?;
    }

    Ok(image)
}

pub async fn update_profile_image(
    locale: &Locale,
    pool: &Pool,
    _: &KvPool,
    account_id: &i64,
    param: UpdateAccountImageParam,
) -> ServiceResult<ProfileImage> {
    let now = Utc::now();
    let cfg = Config::global();
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
    let updated_row = query!(
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
    //
    let image = format_image(DbProfileImage {
        id: updated_row.id,
        account_id: *account_id,
        _order: order,
        url,
        updated_at: now.naive_utc(),
        width,
        height,
        size,
        mime_type,
    });
    if order == 0 {
        update_account_image(pool, *account_id, true, Some(image.clone())).await?;
    } else {
        update_account_image(pool, *account_id, false, None).await?;
    }
    Ok(image)
}

pub async fn delete_profile_image(pool: &Pool, account_id: &i64, order: i16) -> ServiceResult<()> {
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
    if order == 0 {
        update_account_image(pool, *account_id, true, None).await?;
    } else {
        update_account_image(pool, *account_id, false, None).await?;
    }

    Ok(())
}
