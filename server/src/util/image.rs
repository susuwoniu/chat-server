use crate::types::{Image, ImageVersion, ImagesJson, Thumbnail};
use serde_json::Value;
pub fn format_image(url: String, width: f64, height: f64, size: i64, mime_type: String) -> Image {
    let thumbnail_default_width = 300.0;
    let thumbnail_default_height = height * thumbnail_default_width as f64 / width;
    let thumbnail_url = format!("{}/{}", url, "thumbnail");

    let large_default_width: f64 = 1024.0;
    let large_default_height = height * large_default_width as f64 / width;
    let large_url = format!("{}/{}", url, "large");

    return Image {
        url: url,
        width: width,
        height: height,
        size: size,
        mime_type: mime_type.clone(),
        large: Thumbnail {
            url: large_url,
            width: large_default_width,
            height: large_default_height,
            mime_type: mime_type.clone(),
        },
        thumbnail: Thumbnail {
            url: thumbnail_url,
            width: thumbnail_default_width as f64,
            height: thumbnail_default_height,
            mime_type: mime_type,
        },
    };
}
pub fn format_avatar(url: String, width: f64, height: f64, size: i64, mime_type: String) -> Image {
    let thumbnail_default_width = 300.0;
    let thumbnail_default_height = height * thumbnail_default_width as f64 / width;
    let thumbnail_url = format!("{}/{}", url, "avatar");

    let large_default_width: f64 = 1024.0;
    let large_default_height = height * large_default_width as f64 / width;
    let large_url = format!("{}/{}", url, "large");

    return Image {
        url: url,
        width: width,
        height: height,
        size: size,
        mime_type: mime_type.clone(),
        large: Thumbnail {
            url: large_url,
            width: large_default_width,
            height: large_default_height,
            mime_type: mime_type.clone(),
        },
        thumbnail: Thumbnail {
            url: thumbnail_url,
            width: thumbnail_default_width as f64,
            height: thumbnail_default_height,
            mime_type: mime_type,
        },
    };
}

pub fn format_images(db_images_value: Option<Value>) -> Vec<Image> {
    let mut images: Vec<Image> = Vec::new();
    if let Some(images_value) = db_images_value {
        let db_images: ImagesJson = serde_json::from_value(images_value).unwrap_or(ImagesJson {
            version: ImageVersion::V1,
            images: Vec::new(),
        });
        images = db_images.images;
    }
    return images;
}
