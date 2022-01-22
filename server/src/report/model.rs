use crate::{
    error::ServiceError,
    util::{base62_to_i64, datetime_tz, option_datetime_tz, option_string_i64, string_i64},
};
use chrono::prelude::NaiveDateTime;
use jsonapi::{api::*, jsonapi_model, model::*};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
// _type: 0: unknown, 1: offensive 辱骂/攻击/冒犯 2: ad 广告 3: spam 垃圾信息 4: porn 色情低俗 5: politics 政治相关 6: illegal 违法违规 7: leak 泄漏他人隐私 8: violate 侵犯我的权益, 9: complaint 其他投诉 80: feedback bug反馈,功能建议, 81: ask 咨询 99: other 其他
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]
pub enum ReportType {
    Unknown = 0,
    Offensive = 1,
    Ad = 2,
    Spam = 3,
    Porn = 4,
    Politics = 5,
    Illegal = 6,
    Leak = 7,
    Violate = 8,
    Complaint = 9,
    Feedback = 80,
    Ask = 81,
    Other = 99,
}

// -- state: 0: open, 1: closed
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]
pub enum ReportState {
    Open = 0,
    Closed = 1,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub content: String,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "type")]
    pub _type: ReportType,
    pub images: Vec<String>,
    pub state: ReportState,
    #[serde(with = "option_string_i64")]
    pub related_post_id: Option<i64>,
    #[serde(with = "option_string_i64")]
    pub related_account_id: Option<i64>,
    #[serde(with = "option_string_i64")]
    pub replied_by: Option<i64>,
    pub replied_content: Option<String>,
    #[serde(with = "option_datetime_tz")]
    pub replied_at: Option<NaiveDateTime>,
}
jsonapi_model!(Report; "reports");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilter {
    pub after: Option<i64>,
    pub before: Option<i64>,
    #[serde(rename = "type")]
    pub _type: Option<ReportType>,
    pub state: Option<ReportState>,
    pub limit: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiReportFilter {
    pub after: Option<String>,
    pub before: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<ReportType>,
    pub state: Option<ReportState>,
    pub limit: Option<i64>,
}
impl TryFrom<ApiReportFilter> for ReportFilter {
    type Error = ServiceError;

    fn try_from(value: ApiReportFilter) -> Result<Self, Self::Error> {
        let mut after = None;
        if let Some(after_value) = value.after {
            after = Some(base62_to_i64(&after_value)?);
        }
        let mut before = None;
        if let Some(before_value) = value.before {
            before = Some(base62_to_i64(&before_value)?);
        }
        Ok(Self {
            limit: value.limit,
            after,
            before,
            _type: value._type,
            state: value.state,
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullReport {
    #[serde(with = "string_i64")]
    pub id: i64,
    pub content: String,
    #[serde(with = "string_i64")]
    pub account_id: i64,
    #[serde(with = "datetime_tz")]
    pub created_at: NaiveDateTime,
    #[serde(with = "datetime_tz")]
    pub updated_at: NaiveDateTime,
    #[serde(rename = "type")]
    pub _type: ReportType,
    pub images: Vec<String>,
    pub state: ReportState,
    #[serde(with = "option_string_i64")]
    pub related_post_id: Option<i64>,
    #[serde(with = "option_string_i64")]
    pub related_account_id: Option<i64>,
    #[serde(with = "option_string_i64")]
    pub replied_by: Option<i64>,
    pub replied_content: Option<String>,
    #[serde(with = "option_datetime_tz")]
    pub replied_at: Option<NaiveDateTime>,
}
jsonapi_model!(FullReport; "full-reports");
#[derive(Debug, Clone)]
pub struct DbReport {
    pub id: i64,
    pub content: String,
    pub account_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub _type: ReportType,
    pub images: Vec<String>,
    pub state: ReportState,
    pub related_post_id: Option<i64>,
    pub related_account_id: Option<i64>,
    pub replied_by: Option<i64>,
    pub replied_content: Option<String>,
    pub replied_at: Option<NaiveDateTime>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReportParam {
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub _type: ReportType,
    pub images: Option<Vec<String>>,
    #[serde(with = "option_string_i64", default)]
    pub related_post_id: Option<i64>,
    #[serde(with = "option_string_i64", default)]
    pub related_account_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReportParam {
    pub state: ReportState,
    pub replied_content: Option<String>,
}

impl From<FullReport> for Report {
    fn from(full: FullReport) -> Self {
        let FullReport {
            id,
            content,
            account_id,
            created_at,
            updated_at,
            _type,
            images,
            state,
            related_post_id,
            related_account_id,
            replied_by,
            replied_content,
            replied_at,
            ..
        } = full;

        Self {
            id,
            content,
            account_id,
            created_at,
            updated_at,
            _type,
            images,
            state,
            related_post_id,
            related_account_id,
            replied_by,
            replied_content,
            replied_at,
        }
    }
}
