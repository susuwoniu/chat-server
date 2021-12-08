use jsonapi::{api::*, jsonapi_model, model::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadSlot {
  pub put_url: String,
  pub get_url: String,
  pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUploadSlot {
  pub mime_type: String,
  pub size: i64,
  pub height: f64,
  pub width: f64,
}
