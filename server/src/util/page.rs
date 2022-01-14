use crate::types::PageInfo;
use axum::Json;
use chrono::Utc;
use jsonapi::model::*;
use serde_json::{json, Value};
use std::collections::HashMap;
pub fn format_response(response: JsonApiDocument) -> Json<JsonApiDocument> {
    // server time
    let server_time = Utc::now();
    let server_time_value = json!(server_time.to_rfc3339());
    let new_response = match response {
        JsonApiDocument::Data(success_data) => {
            let mut new_success_data = success_data.clone();

            let new_meta;
            if let Some(meta) = success_data.meta {
                let mut original_meta = meta.clone();
                original_meta.insert("now".to_string(), server_time_value);
                new_meta = Some(original_meta);
            } else {
                let mut original_meta = HashMap::new();
                original_meta.insert("now".to_string(), server_time_value);
                new_meta = Some(original_meta);
            }
            new_success_data.meta = new_meta;

            JsonApiDocument::Data(new_success_data)
        }
        JsonApiDocument::Error(error_data) => {
            let mut new_error_data = error_data.clone();

            let new_meta;
            if let Some(meta) = error_data.meta {
                let mut original_meta = meta.clone();
                original_meta.insert("now".to_string(), server_time_value);
                new_meta = Some(original_meta);
            } else {
                let mut original_meta = HashMap::new();
                original_meta.insert("now".to_string(), server_time_value);
                new_meta = Some(original_meta);
            }
            new_error_data.meta = new_meta;

            JsonApiDocument::Error(new_error_data)
        }
    };
    return Json(new_response);

    // return Json(response);
}

pub fn format_page_links(
    prefix: &str,
    path: &str,
    query: HashMap<String, String>,
    page_info: PageInfo,
) -> HashMap<String, Value> {
    let current_path = format!("{}{}", prefix, path);
    let PageInfo { start, end } = page_info;
    let mut map: HashMap<String, Value> = HashMap::new();

    let mut prev_value = None;
    if let Some(start) = start {
        let mut current_query = query.clone();
        current_query.insert("before".into(), bs62::encode_num(&start));
        current_query.remove("after");
        let query = serde_urlencoded::to_string(&current_query).unwrap();
        prev_value = Some(json!(format!("{}?{}", current_path, query)));
    }
    map.insert("prev".to_string(), json!(prev_value));
    let mut next_value = None;
    if let Some(end) = end {
        let mut current_query = query.clone();
        current_query.insert("after".into(), bs62::encode_num(&end));
        current_query.remove("before");

        let query = serde_urlencoded::to_string(&current_query).unwrap();
        next_value = Some(json!(format!("{}?{}", current_path.clone(), query)));
    }
    map.insert("next".to_string(), json!(next_value));
    return map;
}
pub fn format_page_meta(page_info: PageInfo) -> HashMap<String, Value> {
    let mut meta: HashMap<String, Value> = HashMap::new();
    meta.insert("page_info".to_string(), json!(page_info));
    return meta;
}
