use crate::notification::model::{NotificationAction, NotificationType};
pub fn notification_type_to_string(_type: &NotificationType) -> String {
    match _type {
        NotificationType::ProfileViewed => "profile_viewed".to_string(),
        NotificationType::ProfileLiked => "profile_liked".to_string(),
        NotificationType::Unknown => "unknown".to_string(),
    }
}

pub fn notification_action_to_string(_type: &NotificationAction) -> String {
    match _type {
        NotificationAction::ProfileViewed => "profile_viewed".to_string(),
        NotificationAction::ProfileLiked => "profile_liked".to_string(),
    }
}
