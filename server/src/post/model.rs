use crate::{
  account::model::Account,
  error::ServiceError,
  types::{FieldAction, Gender, PageInfo},
  util::{
    base62_i64, base62_to_i64, datetime_tz, default, option_base62_i64, option_datetime_tz,
    option_string_i64, string::parse_skip_range, string_i64,
  },
};
use chrono::prelude::{NaiveDate, NaiveDateTime};
use derivative::Derivative;
use ipnetwork17::IpNetwork;
use jsonapi::{api::*, array::JsonApiArray, jsonapi_model, model::*};
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
pub struct PostTemplateFilter {
  #[serde(with = "option_base62_i64")]
  pub after: Option<i64>,
  #[serde(with = "option_base62_i64")]
  pub before: Option<i64>,
  pub featured: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiPostFilter {
  pub after: Option<String>,
  pub before: Option<String>,
  pub skip: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostFilter {
  pub after: Option<i64>,
  pub before: Option<i64>,
  pub skip: Vec<[i64; 2]>,
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
    Ok(PostFilter {
      after,
      before,
      skip,
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
