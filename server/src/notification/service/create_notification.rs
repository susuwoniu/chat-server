use crate::{
    alias::{KvPool, Pool},
    middleware::{Auth, Locale},
    notification::model::CreateNotificationParam,
    types::ServiceResult,
    util::id::next_id,
};
use chrono::Utc;
use serde_json::json;
use sqlx::query;
pub async fn create_notification(
    _: &Locale,
    pool: &Pool,
    _: &KvPool,
    param: CreateNotificationParam,
    auth: Auth,
) -> ServiceResult<()> {
    let CreateNotificationParam {
        content,
        target_account_id,
        _type,
        action,
        action_data,
        is_primary,
    } = param;
    // add notification template
    let id = next_id();
    let now = Utc::now().naive_utc();
    let mut tx = pool.begin().await?;

    query!(
        r#"
INSERT INTO notifications (id,content,account_id,updated_at,_type,_action,action_data,is_primary,from_account_id)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
"#,
        id,
        content,
        target_account_id,
        now,
        json!(_type).to_string(),
        json!(action).to_string(),
        json!(action_data),
        is_primary,
        auth.account_id
    )
    .execute(&mut tx)
    .await?;
    let inbox_id = next_id();

    query!(
        r#"
INSERT into notification_inboxes as t
(id, updated_at,created_at,account_id, _type,unread_count,last_notification_id,last_notification_updated_at,last_notification_from,total_count)
VALUES ($1,$2,$3,$4,$5,1,$6,$7,$8,1) 
ON CONFLICT (account_id,_type)  
DO UPDATE SET 
updated_at=$2,
unread_count=t.unread_count + 1,
last_notification_id=$6,
last_notification_updated_at=$7,
last_notification_from=$8,
total_count = t.total_count + 1
"#,
        inbox_id,
        now,
        now,
        target_account_id,
        json!(_type).to_string(),
        id,
        now,
        auth.account_id
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    return Ok(());
}
