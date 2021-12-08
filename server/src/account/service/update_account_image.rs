use crate::{
  account::model::{DbProfileImage, ProfileImage, Thumbtail, UpdateAccountImageParam},
  alias::{KvPool, Pool},
  error::{Error, ServiceError},
  global::Config,
  im::{model::ImUpdateAccountParam, service::update_im_account::update_im_account},
  middleware::Locale,
  types::ServiceResult,
  util::id::next_id,
};
use chrono::Utc;
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
