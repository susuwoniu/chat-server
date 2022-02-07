use crate::{
    error::{Error, ServiceError},
    file::model::{CreateUploadSlot, UploadSlot},
    global::Config,
    middleware::{Auth, Locale},
    types::ServiceResult,
    util::{convert::header_to_hash_map, crypto::hash, id::next_id},
};
use sonyflake::Sonyflake;
use urlencoding::encode;

pub async fn create_profile_image_upload_slot(
    locale: &Locale,
    param: CreateUploadSlot,
    auth: Auth,
    sf: &mut Sonyflake,
) -> ServiceResult<UploadSlot> {
    let CreateUploadSlot {
        mime_type,
        size,
        height,
        width,
    } = param;
    let cfg = Config::global();
    let bucket_url = cfg.profile_image_file_server.bucket_url.clone();
    let mime_result = mime_type.parse::<mime::Mime>();
    if let Ok(mime) = mime_result {
        if mime.type_() == mime::IMAGE {
            let account_hash = hash(&auth.account_id.to_string());
            let mut file_path = format!(
                "/{}/{}/type/{}/size/{}/width/{}/height/{}",
                account_hash,
                next_id(sf),
                encode(&mime_type),
                size,
                width,
                height
            );
            let suffix = mime.suffix();
            if let Some(suffix) = suffix {
                file_path = format!("{}.{}", file_path, suffix);
            }
            let file_url = format!(
                "{}://{}{}",
                bucket_url.scheme(),
                bucket_url.host_str().unwrap(),
                file_path
            );
            let datetime = chrono::Utc::now();
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", mime_type.parse().unwrap());
            headers.insert("X-Amz-Acl", "public-read".to_string().parse().unwrap());
            headers.insert(
                "X-Amz-Date",
                datetime
                    .format("%Y%m%dT%H%M%SZ")
                    .to_string()
                    .parse()
                    .unwrap(),
            );
            headers.insert("Content-Length", size.to_string().parse().unwrap());
            let s = aws_sign_v4::AwsSign::new(
                "PUT",
                &file_url,
                &datetime,
                &headers,
                &cfg.profile_image_file_server.region,
                &cfg.profile_image_file_server.access_key_id,
                &cfg.profile_image_file_server.access_key_secret,
            );
            let signature = s.sign();
            let signature_string: String = signature.parse().unwrap();
            // Authorization
            let mut header_map = header_to_hash_map(&headers);
            header_map.insert("authorization".to_string(), signature_string);
            return Ok(UploadSlot {
                put_url: file_url.clone(),
                get_url: file_url,
                headers: header_map,
            });
        }
    }
    return Err(ServiceError::param_invalid(
        locale,
        "mime_type_invalid",
        Error::Other(format!("{} is invalid", mime_type)),
    ));
}

pub async fn create_avatar_upload_slot(
    locale: &Locale,
    param: CreateUploadSlot,
    auth: Auth,
    sf: &mut Sonyflake,
) -> ServiceResult<UploadSlot> {
    let CreateUploadSlot {
        mime_type,
        size,
        height,
        width,
    } = param;
    let cfg = Config::global();
    let bucket_url = cfg.avatar_file_server.bucket_url.clone();
    let mime_result = mime_type.parse::<mime::Mime>();
    if let Ok(mime) = mime_result {
        if mime.type_() == mime::IMAGE {
            let account_hash = hash(&auth.account_id.to_string());
            let mut file_path = format!(
                "/{}/{}/type/{}/size/{}/width/{}/height/{}",
                account_hash,
                next_id(sf),
                encode(&mime_type),
                size,
                width,
                height
            );
            let suffix = mime.suffix();
            if let Some(suffix) = suffix {
                file_path = format!("{}.{}", file_path, suffix);
            }
            let file_url = format!(
                "{}://{}{}",
                bucket_url.scheme(),
                bucket_url.host_str().unwrap(),
                file_path
            );
            let datetime = chrono::Utc::now();
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", mime_type.parse().unwrap());
            headers.insert("X-Amz-Acl", "public-read".to_string().parse().unwrap());
            headers.insert(
                "X-Amz-Date",
                datetime
                    .format("%Y%m%dT%H%M%SZ")
                    .to_string()
                    .parse()
                    .unwrap(),
            );
            headers.insert("Content-Length", size.to_string().parse().unwrap());
            let s = aws_sign_v4::AwsSign::new(
                "PUT",
                &file_url,
                &datetime,
                &headers,
                &cfg.profile_image_file_server.region,
                &cfg.profile_image_file_server.access_key_id,
                &cfg.profile_image_file_server.access_key_secret,
            );
            let signature = s.sign();
            let signature_string: String = signature.parse().unwrap();
            // Authorization
            let mut header_map = header_to_hash_map(&headers);
            header_map.insert("authorization".to_string(), signature_string);
            return Ok(UploadSlot {
                put_url: file_url.clone(),
                get_url: file_url,
                headers: header_map,
            });
        }
    }
    return Err(ServiceError::param_invalid(
        locale,
        "mime_type_invalid",
        Error::Other(format!("{} is invalid", mime_type)),
    ));
}
