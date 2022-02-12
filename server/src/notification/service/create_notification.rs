use crate::{
    alias::{KvPool, Pool},
    global::config::Config,
    middleware::{Auth, Locale},
    notification::model::CreateNotificationParam,
    types::{FieldAction, ServiceResult},
    util::id::next_id,
};
use chrono::Utc;
use serde_json::json;
use sonyflake::Sonyflake;
use sqlx::query;
use std::collections::HashMap;
pub async fn create_notification(
    _: &Locale,
    pool: &Pool,
    _: &KvPool,
    param: CreateNotificationParam,
    auth: Auth,
    sf: &mut Sonyflake,
) -> ServiceResult<()> {
    let CreateNotificationParam {
        content,
        target_account_id,
        _type,
        action,
        action_data,
        is_primary,
        field_action,
    } = param;
    // add notification template
    let id = next_id(sf);
    let now = Utc::now().naive_utc();
    let mut tx = pool.begin().await?;
    if field_action == FieldAction::IncreaseOne {
        query!(
            r#"
    INSERT INTO notifications (id,content,account_id,updated_at,_type,_action,action_data,is_primary,from_account_id)
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
    "#,
            id,
            content,
            target_account_id,
            now,
            _type as _,
            action as _,
            json!(action_data),
            is_primary,
            auth.account_id
        )
        .execute(&mut tx)
        .await?;
    }

    let inbox_id = next_id(sf);
    let count_changed_value = if field_action == FieldAction::IncreaseOne {
        1
    } else {
        -1
    };

    query!(
        r#"
INSERT into notification_inboxes as t
(id, updated_at,created_at,account_id, _type,unread_count,last_notification_id,last_notification_updated_at,last_notification_from,total_count)
VALUES ($1,$2,$3,$4,$5,1,$6,$7,$8,1) 
ON CONFLICT (account_id,_type)  
DO UPDATE SET 
updated_at=$2,
unread_count=t.unread_count + $9,
last_notification_id=$6,
last_notification_updated_at=$7,
last_notification_from=$8,
total_count = t.total_count + $9
"#,
        inbox_id,
        now,
        now,
        target_account_id,
        _type as _,
        id,
        now,
        auth.account_id,
        count_changed_value
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    return Ok(());
}

pub async fn error(title: &str, content: String) -> ServiceResult<()> {
    // send to notification service
    // https://maker.ifttt.com/trigger/{event}/with/key/{key}
    let cfg = Config::global();
    let mut map = HashMap::new();
    map.insert("value1", title.to_string());
    map.insert("value2", content);

    let client = reqwest::Client::new();
    client
        .post(format!(
            "https://maker.ifttt.com/trigger/{}/with/key/{}",
            cfg.notification.ifttt_event, cfg.notification.ifttt_key
        ))
        .json(&map)
        .send()
        .await?;
    return Ok(());
}
