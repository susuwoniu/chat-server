use crate::{
    alias::Pool,
    error::{Error, ServiceError},
    middleware::{Auth, Locale},
    report::{
        model::{DbReport, Report, ReportState, ReportType, UpdateReportParam},
        service::get_report::format_report,
    },
    types::ServiceResult,
};

use chrono::Utc;
use sqlx::query_as;
pub async fn update_report(
    locale: &Locale,
    pool: &Pool,
    id: i64,
    param: UpdateReportParam,
    auth: Auth,
    _: bool,
) -> ServiceResult<Report> {
    let UpdateReportParam {
        state,
        replied_content,
    } = param;
    let is_admin = auth.admin;
    let is_moderator = auth.moderator;
    // only admin or moderator
    if !is_admin && !is_moderator {
        return Err(ServiceError::permission_limit(
            locale,
            "no_permission_to_modify_report",
            Error::Default,
        ));
    }
    // get post template
    let now = Utc::now().naive_utc();

    let row =  query_as!(DbReport,
    r#"
        UPDATE reports set 
        state= coalesce($3,state),
        replied_content = coalesce($4,replied_content),
        replied_at = $1,
        replied_by = $5,
        updated_at = $1
        WHERE id = $2
        RETURNING id,_type as "_type:ReportType",content,account_id,updated_at,created_at,images,related_post_id,related_account_id,state as "state:ReportState",replied_by,replied_content,replied_at,note
        "#,
    now,
    id,
    state as _,
    replied_content,
    auth.account_id

  )
  .fetch_one(pool)
  .await?;

    Ok(format_report(row).into())
}
