use crate::{
    error::{Error, ServiceError},
    middleware::header_x_client_platform::XClientPlatform,
    middleware::Locale,
};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]

pub enum ClientPlatform {
    IOS = 1,
    Android = 2,
    #[serde(rename = "iOS")]
    Web = 3,
    Windows = 4,
    #[serde(rename = "macOS")]
    MacOS = 5,
    Linux = 6,
    WechatMini = 7,
}

#[async_trait]
impl<B> FromRequest<B> for ClientPlatform
where
    B: Send,
{
    type Rejection = ServiceError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let header_value = TypedHeader::<XClientPlatform>::from_request(req).await;
        let mut header_value_string: String = "".to_string();
        let locale = Locale::from_request(req).await?;

        if let Ok(TypedHeader(XClientPlatform(header_value))) = header_value {
            // parse language
            header_value_string = header_value.to_lowercase();
        }
        if !header_value_string.is_empty() {
            match header_value_string.as_str() {
                "ios" => Ok(ClientPlatform::IOS),
                "android" => Ok(ClientPlatform::Android),
                _ => Err(ServiceError::bad_request(
                    &locale,
                    "not_support_client_platform",
                    Error::Default,
                )),
            }
        } else {
            Err(ServiceError::bad_request(
                &locale,
                "can_not_found_header_client_platform",
                Error::Default,
            ))
        }
    }
}
