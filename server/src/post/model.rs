use crate::{
  account::model::Account,
  error::ServiceError,
  global::Config,
  types::{FieldAction, Gender},
  util::{
    base62_i64, base62_to_i64, datetime_tz, option_datetime_tz, option_string_i64,
    string::parse_skip_range, string_i64,
  },
};
use chrono::{
  prelude::{NaiveDateTime, Utc},
  Duration,
};
use derivative::Derivative;
use ipnetwork17::IpNetwork;
use jsonapi::{api::*, jsonapi_model, model::*};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTemplate {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub content: String,
  pub used_count: i64,
  pub skipped_count: i64,
  pub background_color: String,
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub featured: bool,
  #[serde(with = "option_datetime_tz")]
  pub featured_at: Option<NaiveDateTime>,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
}
jsonapi_model!(PostTemplate; "post-templates");

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "visibility", rename_all = "lowercase")]
pub enum Visibility {
  Public,
  Unlisted,
  Related,
  Private,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostView {
  #[serde(with = "string_i64")]
  pub id: i64,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  #[serde(with = "string_i64")]
  pub post_id: i64,
  #[serde(with = "string_i64")]
  pub post_account_id: i64,
  #[serde(with = "string_i64")]
  pub viewed_by: i64,
  pub viewed_by_account: Account,
  #[serde(with = "base62_i64")]
  pub cursor: i64,
}
jsonapi_model!(PostView; "post-views"; has one viewed_by_account);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbPostView {
  #[serde(with = "string_i64")]
  pub id: i64,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  #[serde(with = "string_i64")]
  pub post_id: i64,
  #[serde(with = "string_i64")]
  pub post_account_id: i64,
  #[serde(with = "string_i64")]
  pub viewed_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSkip {
  #[serde(with = "string_i64")]
  pub id: i64,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  #[serde(with = "string_i64")]
  pub post_id: i64,
  #[serde(with = "string_i64")]
  pub post_account_id: i64,
  #[serde(with = "string_i64")]
  pub skipped_by: i64,
  pub skipped_by_account: Account,
  #[serde(with = "base62_i64")]
  pub cursor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostReply {
  #[serde(with = "string_i64")]
  pub id: i64,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  #[serde(with = "string_i64")]
  pub post_id: i64,
  #[serde(with = "string_i64")]
  pub post_account_id: i64,
  #[serde(with = "string_i64")]
  pub replied_by: i64,
  pub replied_by_account: Account,
  #[serde(with = "base62_i64")]
  pub cursor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub content: String,
  pub viewed_count: i64,
  pub skipped_count: i64,
  pub replied_count: i64,
  pub background_color: String,
  #[serde(with = "string_i64")]
  pub account_id: i64,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  pub visibility: Visibility,
  pub target_gender: Option<Gender>,
  pub author: Account,
  #[serde(with = "string_i64")]
  pub post_template_id: i64,
  #[serde(with = "base62_i64")]
  pub cursor: i64,
  pub gender: Gender,
}
jsonapi_model!(Post; "posts"; has one author);
#[derive(Debug, Clone)]
pub struct DbPost {
  pub id: i64,
  pub content: String,
  pub viewed_count: i64,
  pub skipped_count: i64,
  pub replied_count: i64,
  pub background_color: String,
  pub account_id: i64,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub visibility: Visibility,
  pub target_gender: Option<Gender>,
  pub post_template_id: i64,
  pub time_cursor: i64,
  pub gender: Gender,
  pub client_id: i64,
  pub ip: Option<IpNetwork>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPostTemplateFilter {
  pub after: Option<String>,
  pub before: Option<String>,
  pub featured: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTemplateFilter {
  pub after: Option<i64>,
  pub before: Option<i64>,
  pub featured: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiPostFilter {
  pub after: Option<String>,
  pub before: Option<String>,
  pub skip: Option<Vec<String>>,
  pub start_time: Option<NaiveDateTime>,
  pub end_time: Option<NaiveDateTime>,
  pub account_id: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostFilter {
  pub after: Option<i64>,
  pub before: Option<i64>,
  pub skip: Vec<[i64; 2]>,
  pub start_time: Option<NaiveDateTime>,
  pub end_time: Option<NaiveDateTime>,
  pub account_id: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiPostViewFilter {
  pub after: Option<String>,
  pub before: Option<String>,
  pub start_time: Option<NaiveDateTime>,
  pub end_time: Option<NaiveDateTime>,
  pub post_account_id: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostViewFilter {
  pub after: Option<i64>,
  pub before: Option<i64>,
  pub start_time: Option<NaiveDateTime>,
  pub end_time: Option<NaiveDateTime>,
  pub post_account_id: Option<i64>,
}

impl TryFrom<ApiPostFilter> for PostFilter {
  type Error = ServiceError;

  fn try_from(value: ApiPostFilter) -> Result<Self, Self::Error> {
    let mut after = None;
    if let Some(after_value) = value.after {
      after = Some(base62_to_i64(&after_value)?);
    }
    let mut before = None;
    if let Some(before_value) = value.before {
      before = Some(base62_to_i64(&before_value)?);
    }
    let mut skip = Vec::new();
    if let Some(skip_value) = value.skip {
      skip = parse_skip_range(&skip_value)?;
    }
    let mut start_time = value.start_time;
    if value.start_time.is_none() {
      let cfg = Config::global();
      let days = cfg.post.default_listed_posts_duration_in_days;
      let duration = Duration::days(days);
      let now = Utc::now().naive_utc();
      //start_time 默认一个月内的帖子，减少服务器消耗
      start_time = Some(now - duration);
    }
    Ok(PostFilter {
      after,
      before,
      skip,
      start_time: start_time,
      end_time: value.end_time,
      account_id: value.account_id,
    })
  }
}
impl TryFrom<ApiPostViewFilter> for PostViewFilter {
  type Error = ServiceError;

  fn try_from(value: ApiPostViewFilter) -> Result<Self, Self::Error> {
    let mut after = None;
    if let Some(after_value) = value.after {
      after = Some(base62_to_i64(&after_value)?);
    }
    let mut before = None;
    if let Some(before_value) = value.before {
      before = Some(base62_to_i64(&before_value)?);
    }
    Ok(Self {
      after,
      before,
      start_time: value.start_time,
      end_time: value.end_time,
      post_account_id: value.post_account_id,
    })
  }
}
impl TryFrom<ApiPostTemplateFilter> for PostTemplateFilter {
  type Error = ServiceError;

  fn try_from(value: ApiPostTemplateFilter) -> Result<Self, Self::Error> {
    let mut after = None;
    if let Some(after_value) = value.after {
      after = Some(base62_to_i64(&after_value)?);
    }
    let mut before = None;
    if let Some(before_value) = value.before {
      before = Some(base62_to_i64(&before_value)?);
    }
    Ok(Self {
      after,
      before,
      featured: value.featured,
    })
  }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullPostTemplate {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub content: String,
  pub used_count: i64,
  pub skipped_count: i64,
  pub background_color: String,
  #[serde(with = "string_i64")]
  pub account_id: i64,
  pub featured: bool,
  #[serde(with = "option_string_i64")]
  pub featured_by: Option<i64>,
  #[serde(with = "option_datetime_tz")]
  pub featured_at: Option<NaiveDateTime>,
  #[serde(with = "datetime_tz")]
  pub created_at: NaiveDateTime,
  #[serde(with = "datetime_tz")]
  pub updated_at: NaiveDateTime,
  #[serde(with = "base62_i64")]
  pub cursor: i64,
}
jsonapi_model!(FullPostTemplate; "full-post-templates");
#[derive(Debug, Clone)]
pub struct DbPostTemplate {
  pub id: i64,
  pub content: String,
  pub used_count: i64,
  pub skipped_count: i64,
  pub background_color: String,
  pub account_id: i64,
  pub featured: bool,
  pub featured_by: Option<i64>,
  pub featured_at: Option<NaiveDateTime>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub time_cursor: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostTemplateParam {
  pub content: String,
  pub background_color: String,
  pub featured: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct CreatePostParam {
  pub content: String,
  #[serde(with = "string_i64")]
  pub post_template_id: i64,
  pub background_color: Option<String>,
  pub target_gender: Option<Gender>,
  #[serde(default = "default_visibility")]
  #[derivative(Default(value = "Visibility::Public"))]
  pub visibility: Visibility,
}
fn default_visibility() -> Visibility {
  Visibility::Public
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePostTemplateParam {
  pub content: Option<String>,
  pub background_color: Option<String>,
  pub featured: Option<bool>,
  pub deleted: Option<bool>,
  pub priority: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePostParam {
  pub promote: Option<bool>,
  pub viewed_count_action: Option<FieldAction>,
  pub skipped_count_action: Option<FieldAction>,
  pub featured: Option<bool>,
  pub approved: Option<bool>,
  pub visibility: Option<Visibility>,
  pub deleted: Option<bool>,
  pub replied_count_action: Option<FieldAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCreatePostTemplateParam {
  pub content: String,
  pub background_color: String,
  pub featured: Option<bool>,
}
impl From<FullPostTemplate> for PostTemplate {
  fn from(full: FullPostTemplate) -> Self {
    let FullPostTemplate {
      id,
      content,
      used_count,
      skipped_count,
      background_color,
      account_id,
      featured,
      featured_at,
      created_at,
      updated_at,
      ..
    } = full;

    Self {
      id,
      content,
      used_count,
      skipped_count,
      background_color,
      account_id,
      featured,
      featured_at,
      created_at,
      updated_at,
    }
  }
}