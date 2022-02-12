use crate::types::Image;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageSlot {
    pub put_url: String,
    pub get_url: String,
    pub headers: HashMap<String, String>,
    pub image: Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUploadImageSlot {
    pub mime_type: String,
    pub size: i64,
    pub height: f64,
    pub width: f64,
}
