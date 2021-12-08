use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
pub fn header_to_hash_map(headers: &HeaderMap<HeaderValue>) -> HashMap<String, String> {
  let mut header_hashmap = HashMap::new();
  for (k, v) in headers {
    let k = k.as_str().to_owned();
    let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
    header_hashmap.insert(k, v);
  }
  header_hashmap
}
