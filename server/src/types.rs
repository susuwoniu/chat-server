use crate::error::ServiceError;
use axum::Json;
use jsonapi::api::{DocumentData, JsonApiDocument, Meta, PrimaryData};
use std::collections::HashMap;
// original type common type
pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
pub type ServiceJson<V> = std::result::Result<Json<V>, ServiceError>;
pub type JsonApiResponse = ServiceJson<JsonApiDocument>;
pub type SimpleMetaResponse<T> = ServiceJson<SimpleMetaDoc<T>>;

use serde::{Deserialize, Serialize};
use serde_json::json;
pub struct QuickResponse;
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleMetaDoc<T>
where
  T: Serialize,
{
  meta: T,
}

impl QuickResponse {
  pub fn minimize() -> ServiceJson<JsonApiDocument> {
    let mut success_meta: Meta = HashMap::new();
    success_meta.insert("success".to_string(), json!(true));
    return Ok(Json(JsonApiDocument::Data(DocumentData {
      data: Some(PrimaryData::None),
      ..Default::default()
    })));
  }
  pub fn default() -> ServiceJson<JsonApiDocument> {
    Self::minimize()
  }

  pub fn meta<T>(meta: T) -> SimpleMetaResponse<T>
  where
    T: Serialize,
  {
    return Ok(Json(SimpleMetaDoc { meta }));
  }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
  AddAccountName,
  AddAccountBio,
  AddAccountBirthday,
  AddAccountProfileImage,
  AddAccountGender,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
  pub action_type: ActionType,
  pub required: bool,
}
