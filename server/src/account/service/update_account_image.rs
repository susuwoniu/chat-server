use crate::{
  account::model::{
    DbProfileImage, ProfileImage, Thumbtail, UpdateAccountImageParam, UpdateAccountImagesParam,
  },
  alias::{KvPool, Pool},
  error::{Error, ServiceError},
  global::Config,
  im::{model::ImUpdateAccountParam, service::update_im_account::update_im_account},
  middleware::Locale,
  types::ServiceResult,
  util::id::next_id,
};
use chrono::{NaiveDateTime, Utc};
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
    sequence: image.sequence,
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
      select id,account_id,sequence,url,size,height,width,mime_type,updated_at from account_images
      where account_id = $1
  "#,
    account_id,
  )
  .fetch_all(pool)
  .await?;

  Ok(images.into_iter().map(format_image).collect())
}
async fn update_avatar(
  pool: &Pool,
  kv: &KvPool,
  account_id: i64,
  image: ProfileImage,
) -> ServiceResult<()> {
  let now = Utc::now();
  let avatar_url = format!("{}/{}", image.url, "avatar");

  query!(
    r#"
    UPDATE accounts
    SET 
    updated_at=$2,
    avatar_updated_at=$2,
    avatar = $3,
    profile_image_change_count=profile_image_change_count+1
    where id = $1
"#,
    account_id,
    now.naive_utc(),
    &avatar_url,
  )
  .execute(pool)
  .await?;
  // update im avatar
  // TODO
  dbg!("avatar_url", &avatar_url);
  update_im_account(
    kv,
    ImUpdateAccountParam {
      account_id,
      avatar: Some(avatar_url),
      ..Default::default()
    },
  )
  .await?;

  Ok(())
}
pub async fn put_profile_images(
  _: &Locale,
  pool: &Pool,
  account_id: &i64,
  param: UpdateAccountImagesParam,
) -> ServiceResult<Vec<ProfileImage>> {
  let now = Utc::now();

  // 事务
  let mut tx = pool.begin().await?;
  let _ = query!(
    r#"
      DELETE from account_images
      where account_id = $1
  "#,
    account_id,
  )
  .execute(&mut tx)
  .await;
  let images = param.images;
  let mut db_images: Vec<ProfileImage> = Vec::new();
  // insert all
  let mut v1: Vec<i64> = Vec::new();
  let mut v2: Vec<NaiveDateTime> = Vec::new();
  let mut v3: Vec<i64> = Vec::new();
  let mut v4: Vec<i32> = Vec::new();
  let mut v5: Vec<String> = Vec::new();
  let mut v6: Vec<f64> = Vec::new();
  let mut v7: Vec<f64> = Vec::new();
  let mut v8: Vec<i64> = Vec::new();
  let mut v9: Vec<String> = Vec::new();
  let id = next_id();
  let mut index = 0;
  images.into_iter().for_each(|row| {
    let final_id = id + index;
    v1.push(final_id);
    index = index + 1;
    v2.push(now.naive_utc());
    v3.push(*account_id);
    v4.push(row.sequence);
    v5.push(row.url.clone());
    v6.push(row.width);
    v7.push(row.height);
    v8.push(row.size);
    v9.push(row.mime_type.clone());
    db_images.push(format_image(DbProfileImage {
      id: final_id,
      account_id: *account_id,
      sequence: row.sequence,
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
    (id, updated_at, account_id, sequence, url, width, height, size, mime_type) SELECT * FROM UNNEST ($1,$2,$3,$4,$5,$6,$7,$8,$9)
"#
  ).bind(&v1).bind(&v2).bind(&v3).bind(&v4).bind(&v5).bind(&v6).bind(&v7).bind(&v8).bind(&v9)
  .execute(&mut tx)
  .await?;

  // update avatar
  // let image = format_image(DbProfileImage {
  //   id,
  //   account_id: *account_id,
  //   sequence,
  //   url,
  //   updated_at: now.naive_utc(),
  //   width,
  //   height,
  //   size,
  //   mime_type,
  // });
  // if sequence == 0 {
  //   update_avatar(pool, kv, *account_id, image.clone()).await?;
  // }
  tx.commit().await?;
  return Ok(db_images);
}

pub async fn insert_or_update_profile_image(
  locale: &Locale,
  pool: &Pool,
  kv: &KvPool,
  account_id: &i64,
  param: UpdateAccountImageParam,
) -> ServiceResult<ProfileImage> {
  let now = Utc::now();
  let id = next_id();
  let UpdateAccountImageParam {
    sequence,
    url,
    width,
    height,
    size,
    mime_type,
  } = param;
  let cfg = Config::global();
  if sequence >= cfg.account.max_profile_images {
    return Err(ServiceError::bad_request(
      locale,
      "reach_account_images_limit",
      Error::Other(format!("sequence: {}", sequence)),
    ));
  }
  query!(
    r#"
    INSERT into account_images 
    (id, updated_at, account_id, sequence, url, width, height, size, mime_type)
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
    ON CONFLICT (account_id, sequence)  DO UPDATE SET 
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
    sequence,
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
    sequence,
    url,
    updated_at: now.naive_utc(),
    width,
    height,
    size,
    mime_type,
  });
  if sequence == 0 {
    update_avatar(pool, kv, *account_id, image.clone()).await?;
  }

  Ok(image)
}

pub async fn update_profile_image(
  locale: &Locale,
  pool: &Pool,
  kv: &KvPool,
  account_id: &i64,
  param: UpdateAccountImageParam,
) -> ServiceResult<ProfileImage> {
  let now = Utc::now();
  let cfg = Config::global();
  let UpdateAccountImageParam {
    sequence,
    url,
    width,
    height,
    size,
    mime_type,
  } = param;
  if sequence >= cfg.account.max_profile_images {
    return Err(ServiceError::bad_request(
      locale,
      "reach_account_images_limit",
      Error::Other(format!("sequence: {}", sequence)),
    ));
  }
  let updated_row = query!(
    r#"
    UPDATE account_images
    SET 
    updated_at=$3,
    url=COALESCE($4,url),
    sequence =$2,
    width = COALESCE($5,width),
    height= COALESCE($6,height),
    size= COALESCE($7,size),
    mime_type = COALESCE($8,mime_type)
    where account_id = $1 and sequence = $2
    RETURNING id
"#,
    account_id,
    sequence,
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
    sequence,
    url,
    updated_at: now.naive_utc(),
    width,
    height,
    size,
    mime_type,
  });
  if sequence == 0 {
    update_avatar(pool, kv, *account_id, image.clone()).await?;
  }
  Ok(image)
}

pub async fn delete_profile_image(
  pool: &Pool,
  account_id: &i64,
  sequence: i32,
) -> ServiceResult<()> {
  let _ = query!(
    r#"
    DELETE from account_images
    where account_id = $1 and sequence = $2
"#,
    account_id,
    sequence,
  )
  .execute(pool)
  .await;
  Ok(())
}
