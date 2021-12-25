use crate::{
    alias::Pool,
    middleware::{Auth, Locale},
    notification::model::{
        DbNotificationInbox, NotificationInbox, NotificationInboxFilter, NotificationInboxItem,
        NotificationType,
    },
    types::ServiceResult,
};
use chrono::Utc;
use sqlx::query_as;
use std::collections::HashMap;
pub async fn get_notification_inbox(
    _: &Locale,
    pool: &Pool,
    auth: Auth,
    _: NotificationInboxFilter,
) -> ServiceResult<NotificationInbox> {
    let now = Utc::now().naive_utc();
    let rows = query_as!(
        DbNotificationInbox,
        r#"
      select created_at,updated_at,account_id,_type as "_type:NotificationType",is_primary,unread_count,last_notification_id,last_notification_updated_at,total_count from notification_inboxes where 
      account_id = $1
"#,
        auth.account_id
    )
    .fetch_all(pool)
    .await?;

    // if let Some(with_last_notification) = filter.with_last_notification {
    //   // with last notification
    // }
    // fetch all account
    let mut inbox_map: HashMap<NotificationType, DbNotificationInbox> = HashMap::new();
    for inbox in rows {
        inbox_map.insert(inbox._type.clone(), inbox);
    }
    let profile_viewed_notification_option = inbox_map.remove(&NotificationType::ProfileViewed);
    let profile_liked_notification_option = inbox_map.remove(&NotificationType::ProfileLiked);
    let mut profile_viewed_notification_inbox = NotificationInboxItem {
        account_id: auth.account_id,
        created_at: now,
        updated_at: now,
        _type: NotificationType::ProfileViewed,
        is_primary: false,
        unread_count: 0,
        total_count: 0,
        last_notification: None,
    };
    let mut profile_liked_notification_inbox = NotificationInboxItem {
        account_id: auth.account_id,
        created_at: now,
        updated_at: now,
        _type: NotificationType::ProfileLiked,
        is_primary: true,
        unread_count: 0,
        total_count: 0,
        last_notification: None,
    };
    if let Some(profile_viewed_notification) = profile_viewed_notification_option {
        profile_viewed_notification_inbox =
            format_notification_inbox_item(profile_viewed_notification);
    }
    if let Some(profile_liked_notification) = profile_liked_notification_option {
        profile_liked_notification_inbox =
            format_notification_inbox_item(profile_liked_notification);
    }
    let notification_inbox = NotificationInbox {
        profile_viewed: profile_viewed_notification_inbox,
        profile_liked: profile_liked_notification_inbox,
    };
    return Ok(notification_inbox);
}

fn format_notification_inbox_item(item: DbNotificationInbox) -> NotificationInboxItem {
    let DbNotificationInbox {
        account_id,
        created_at,
        updated_at,
        _type,
        is_primary,
        unread_count,
        last_notification_id: _,
        last_notification_updated_at: _,
        total_count,
    } = item;
    return NotificationInboxItem {
        account_id: account_id,
        created_at: created_at,
        updated_at,
        _type,
        is_primary,
        unread_count,
        last_notification: None,
        total_count,
    };
}
