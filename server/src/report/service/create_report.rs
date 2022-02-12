use crate::{
    alias::{KvPool, Pool},
    middleware::{Auth, Locale},
    report::model::{CreateReportParam, Report, ReportState, ReportType},
    types::{ImageVersion, ImagesJson, ServiceResult},
    util::{id::next_id, image::format_images},
};
use chrono::Utc;
use serde_json::json;
use sonyflake::Sonyflake;
use sqlx::query;
pub async fn create_report(
    _: &Locale,
    pool: &Pool,
    _: &KvPool,
    param: CreateReportParam,
    auth: Auth,
    sf: &mut Sonyflake,
) -> ServiceResult<Report> {
    // add post template
    let CreateReportParam {
        content,
        _type,
        images,
        related_post_id,
        related_account_id,
    } = param;
    let id = next_id(sf);
    let now = Utc::now().naive_utc();
    let mut final_images = None;
    if let Some(images) = images {
        final_images = Some(json!(ImagesJson {
            version: ImageVersion::V1,
            images,
        }));
    }

    let final_content = content.unwrap_or_default();
    query!(
        r#"
INSERT INTO reports (id,_type,content,account_id,updated_at,images,related_post_id,related_account_id)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
"#,
        id,
        _type.clone() as ReportType,
        final_content.clone(),
        auth.account_id,
        now,
        final_images,
        related_post_id,
        related_account_id
    )
    .execute(pool)
    .await?;
    // update account post template count

    return Ok(Report {
        id,
        content: final_content,
        account_id: auth.account_id,
        updated_at: now,
        created_at: now,
        _type: _type,
        state: ReportState::Open,
        images: format_images(final_images),
        related_post_id,
        related_account_id,
        replied_by: None,
        replied_content: None,
        replied_at: None,
    });
}
