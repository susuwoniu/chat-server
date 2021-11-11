use crate::{
  types::Action,
  util::{datetime_tz, default, option_datetime_tz, option_string_i64, string_i64},
};
use chrono::prelude::{NaiveDate, NaiveDateTime};
use derivative::Derivative;
use ipnetwork17::IpNetwork;
use jsonapi::{api::*, array::JsonApiArray, jsonapi_model, model::*};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTemplate {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub content: String,
  pub used_count: i64,
  pub skip_count: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct PostTemplateFilter {
  pub since_id: Option<i64>,
  pub until_id: Option<i64>,
  #[serde(default = "default::default_true")]
  #[derivative(Default(value = "true"))]
  pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullPostTemplate {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub content: String,
  pub used_count: i64,
  pub skip_count: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostTemplateParam {
  pub content: String,
  pub background_color: String,
  pub account_id: i64,
  pub featured: Option<bool>,
  pub ip: IpNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostTemplateParam {
  #[serde(with = "string_i64")]
  pub id: i64,
  pub content: Option<String>,
  pub background_color: Option<String>,
  pub featured: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUpdatePostTemplateParam {
  pub content: Option<String>,
  pub background_color: Option<String>,
  pub featured: Option<bool>,
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
      skip_count,
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
      skip_count,
      background_color,
      account_id,
      featured,
      featured_at,
      created_at,
      updated_at,
    }
  }
}
