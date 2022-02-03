use crate::{error::ServiceError, util::option_base62_i64};
use axum::Json;
use chrono::Utc;
use jsonapi::api::{DocumentData, JsonApiDocument, Meta, PrimaryData};
use serde_repr::{Deserialize_repr, Serialize_repr};
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
        let server_time = Utc::now();
        let server_time_value = json!(server_time.to_rfc3339());
        success_meta.insert("now".to_string(), server_time_value);
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
        // todo add now
        return Ok(Json(SimpleMetaDoc { meta }));
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    AgreeCommunityRules,
    AddAccountName,
    AddAccountBio,
    AddAccountBirthday,
    AddAccountProfileImage,
    AddAccountGender,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    #[serde(rename = "type")]
    pub _type: ActionType,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FieldAction {
    IncreaseOne,
    DecreaseOne,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageInfo {
    #[serde(with = "option_base62_i64")]
    pub start: Option<i64>,
    #[serde(with = "option_base62_i64")]
    pub end: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWithPageInfo<T> {
    pub data: Vec<T>,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWithMeta<T, M>
where
    M: Serialize,
{
    pub data: T,
    pub meta: M,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    #[serde(with = "option_base62_i64")]
    pub after: Option<i64>,
    #[serde(with = "option_base62_i64")]
    pub before: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
    Intersex = 3,
    Other = 10,
}

impl Default for Gender {
    fn default() -> Self {
        return Gender::Unknown;
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone)]
#[repr(i32)]
pub enum JsonVersion {
    V1 = 1,
}
