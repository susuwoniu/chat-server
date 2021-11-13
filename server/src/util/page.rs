use crate::types::PageInfo;
use serde_json::{json, Value};
use std::collections::HashMap;
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
