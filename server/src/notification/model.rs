use crate::{
    account::model::Account,
    error::ServiceError,
    global::Config,
    types::{FieldAction, ImageVersion},
    util::{base62_i64, base62_to_i64, datetime_tz, string_i64},
};
use chrono::{
    prelude::{NaiveDateTime, Utc},
    Duration,
};
use jsonapi::{api::*, jsonapi_model, model::*};
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type, Hash, std::cmp::Eq)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]
pub enum NotificationType {
    ProfileViewed,
    ProfileLiked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct AlertParam {
    pub title: Option<String>,
    pub body: String,
    pub badge: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PushForwardAlertParam {
    pub title: Option<String>,
    pub body: String,
    pub badge: i32,
    pub tag: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PushForwardPayloadParam {
    pub service: String,
    pub alert: PushForwardAlertParam,
    pub priority: Option<String>,
    pub mode: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ApiPushParam {
    pub priority: Option<i32>,
    pub alert: AlertParam,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PushParam {
    #[serde(with = "string_i64")]
    pub account_id: i64,
    pub priority: Option<i32>,
    pub alert: AlertParam,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]
pub enum NotificationAction {
    ProfileViewed,
    ProfileLiked,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileViewedActionData {
    pub version: ImageVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileLikeedActionData {
    pub version: ImageVersion,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum NotificationActionData {
    ProfileViewed(ProfileViewedActionData),
    ProfileLiked(ProfileLikeedActionData),
}
// TODO test with data

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub content: String,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    pub from: Account,
    #[serde(with = "base62_i64")]
    pub cursor: i64,
    #[serde(rename = "type")]
    pub _type: NotificationType,
    #[sqlx(rename = "_action")]
    pub action: NotificationAction,
    pub action_data: NotificationActionData,
    pub is_primary: bool,
}
jsonapi_model!(Notification; "notifications"; has one from);
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbNotification {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub content: String,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    pub from: Account,
    #[serde(with = "base62_i64")]
    pub cursor: i64,
    #[serde(rename = "type")]
    pub _type: NotificationType,
    #[sqlx(rename = "_action")]
    pub action: NotificationAction,
    pub action_data: NotificationActionData,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationInboxItem {
    #[serde(with = "string_i64")]
    pub account_id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "type")]
    pub _type: NotificationType,
    pub unread_count: i64,
    pub is_primary: bool,
    pub last_notification: Option<Notification>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationInbox {
    pub profile_viewed: NotificationInboxItem,
    pub profile_liked: NotificationInboxItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbNotificationInbox {
    pub account_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub _type: NotificationType,
    pub is_primary: bool,
    pub unread_count: i64,
    pub last_notification_id: i64,
    pub total_count: i64,
    pub last_notification_updated_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationInboxFilter {
    pub with_last_notification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnreadCountAction {
    Clear,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotificationInboxParam {
    pub unread_count_action: Option<UnreadCountAction>,
    #[serde(rename = "type")]
    pub _type: NotificationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiNotificationFilter {
    pub after: Option<String>,
    pub before: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub account_id: Option<i64>,
    pub limit: Option<i64>,
    #[serde(rename = "type")]
    pub _type: Option<NotificationType>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationFilter {
    pub after: Option<i64>,
    pub before: Option<i64>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub account_id: Option<i64>,
    pub limit: Option<i64>,
    #[serde(rename = "type")]
    pub _type: Option<NotificationType>,
}

impl TryFrom<ApiNotificationFilter> for NotificationFilter {
    type Error = ServiceError;

    fn try_from(value: ApiNotificationFilter) -> Result<Self, Self::Error> {
        let mut after = None;
        if let Some(after_value) = value.after {
            after = Some(base62_to_i64(&after_value)?);
        }
        let mut before = None;
        if let Some(before_value) = value.before {
            before = Some(base62_to_i64(&before_value)?);
        }

        let now = Utc::now().naive_utc();

        let mut start_time = value.start_time;
        if value.start_time.is_none() {
            let cfg = Config::global();
            let days = cfg
                .notification
                .default_listed_notifications_duration_in_days;
            let duration = Duration::days(days);
            //start_time 默认一个月内的帖子，减少服务器消耗
            start_time = Some(now - duration);
        }

        Ok(NotificationFilter {
            limit: value.limit,
            after,
            before,
            start_time: start_time,
            end_time: value.end_time,
            account_id: value.account_id,
            _type: value._type,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateNotificationParam {
    pub content: String,
    pub target_account_id: i64,
    #[serde(rename = "type")]
    pub _type: NotificationType,
    pub field_action: FieldAction,
    #[sqlx(rename = "_action")]
    pub action: NotificationAction,
    pub action_data: NotificationActionData,
    pub is_primary: bool,
}
