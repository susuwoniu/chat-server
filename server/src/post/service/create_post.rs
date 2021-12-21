use crate::{
    account::{
        model::UpdateAccountParam,
        service::{get_account::get_db_account, update_account::update_account},
    },
    alias::{KvPool, Pool},
    middleware::{Auth, Locale},
    post::{
        model::{CreatePostParam, DbPost, Post, Visibility},
        service::{get_post::format_post, get_post_template::get_post_template},
        util,
    },
    types::{FieldAction, Gender, ServiceResult},
    util::id::next_id,
};
use chrono::Utc;
use ipnetwork17::IpNetwork;
use sqlx::query_as;
pub async fn create_post(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: CreatePostParam,
    auth: Auth,
    ip: IpNetwork,
) -> ServiceResult<Post> {
    let CreatePostParam {
        content,
        background_color,
        post_template_id,
        visibility,
        target_gender,
    } = param;
    // add post template
    let id = next_id();
    let now = Utc::now().naive_utc();

    util::is_post_content_valid(locale, &content)?;
    let final_visibility = util::get_post_content_visibility(&content, visibility);

    // get post template info
    let post_template = get_post_template(locale, pool, post_template_id).await?;
    // get account

    let author = get_db_account(locale, pool, auth.account_id).await?;

    let mut final_background_color = post_template.background_color;
    if let Some(background_color) = background_color {
        final_background_color = background_color;
    }
    let post = query_as!(DbPost,
    r#"
INSERT INTO posts (id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender,target_gender,visibility,approved,approved_at,approved_by,birthday)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
RETURNING id,content,background_color,account_id,updated_at,post_template_id,client_id,time_cursor,ip,gender as "gender:Gender",target_gender as "target_gender:Gender",visibility as "visibility:Visibility",created_at,skipped_count,viewed_count,replied_count
"#,
    id,
    content,
    final_background_color,
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
    author.birthday
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
            post_count_action: Some(FieldAction::IncreaseOne),
            ..Default::default()
        },
        &auth,
        true,
    )
    .await?;
    return Ok(format_post(post, account.into()));
}
