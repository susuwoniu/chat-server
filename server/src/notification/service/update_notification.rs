use crate::{
    alias::Pool,
    middleware::{Auth, Locale},
    notification::model::UpdateNotificationInboxParam,
    types::ServiceResult,
};
use chrono::Utc;

use sqlx::query;
pub async fn update_notification_inbox(
    _: &Locale,
    pool: &Pool,
    param: UpdateNotificationInboxParam,
    auth: Auth,
) -> ServiceResult<()> {
    let UpdateNotificationInboxParam {
        unread_count_action,
        _type,
    } = param;
    // add notification template
    let now = Utc::now().naive_utc();
    let mut unread_count = None;
    if let Some(_) = unread_count_action {
        // 暂时只有1种情况，不需要判断
        unread_count = Some(0);
    }
    query!(
        r#"
UPDATE notification_inboxes set
updated_at=$3,
unread_count=COALESCE($4,unread_count)
where
account_id = $1 and _type = $2
"#,
        auth.account_id,
        _type as _,
        now,
        unread_count
    )
    .execute(pool)
    .await?;

    return Ok(());
}
