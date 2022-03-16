use crate::{
    account::service::devices::get_devices_by_account_id,
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::config::Config,
    middleware::{Auth, Locale},
    notification::model::{
        AlertParam, CreateNotificationParam, PushForwardPayloadParam, PushParam,
    },
    types::{FieldAction, ServiceResult},
    util::id::next_id,
};
use serde_json::json;
/**
POST /v3/push HTTP/1.1
Host: api.jpush.cn
Authorization: Basic xxx
Content-Type: application/json
Content-Length: 237

{
    "platform":["android"],
    "audience":{
        "registration_id":["xxxx"]
    },
   "notification" : {
        "android":{
            "alert": "Hi, JPush!",
            "title": "Send to Android"
        }
    }
 */
use std::collections::HashMap;

pub async fn push_forward(
    registration_id: String,
    param: PushForwardPayloadParam,
) -> ServiceResult<reqwest::Response> {
    let cfg = Config::global();
    let PushForwardPayloadParam {
        priority,
        service,
        alert,
    } = param;
    let AlertParam { title, body, badge } = alert;
    let mut json_value = json!({});

    if service == "fcm".to_string() {
        json_value = json!({
            "platform":["android"],
            "audience":{
                "registration_id":[registration_id]
            },
            "notification" : {
                "android":{
                    "alert": body,
                    "title": title.unwrap_or("".to_string()),
                    "badge_add_num": badge,
                    "priority":priority.unwrap_or(0)
                }
            }
        });
    } else if service == "apns".to_string() {
        json_value = json!({
            "platform":["ios"],
            "audience":{
                "registration_id":[registration_id]
            },
            "notification" : {
                "ios":{
                  "alert": {
                    "body": body,
                    "title": title.unwrap_or("".to_string()),
                  },

                    "badge": badge
                }
            }
        });
    } else {
        return Err(ServiceError::bad_request(
            &Locale::default(),
            "service invalid",
            Error::Default,
        ));
    }

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.jpush.cn/v3/push")
        .header("Authorization", &cfg.auth.jiguang_authorization)
        .json(&json_value)
        .send()
        .await?;
    return Ok(res);
}

pub async fn push_by_account_id(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    auth: Auth,
    param: PushParam,
) -> ServiceResult<()> {
    let cfg = Config::global();
    let PushParam {
        priority,
        alert,
        account_id,
    } = param;
    let AlertParam { title, body, badge } = alert;
    let Auth { admin, .. } = auth;
    if !admin {
        return Err(ServiceError::permission_limit(
            locale,
            "permission denied, only admin can push",
            Error::Default,
        ));
    }
    // get devices

    let devices = get_devices_by_account_id(locale, pool, kv, account_id).await?;

    let registration_ids = devices
        .iter()
        .map(|x| x.device_token.clone())
        .collect::<Vec<String>>();
    let title_str = title.unwrap_or("".to_string());
    let json_value = json!({
        "platform":"all",
        "audience":{
            "registration_id":registration_ids
        },
        "notification" : {
            "ios":{
              "alert": {
                "body": body,
                "title": title_str.clone(),
              },
                "badge": badge
            },
              "android":{
                  "alert": body,
                  "title": title_str,
                  "badge_add_num": badge,
                  "priority":priority.unwrap_or(0)
              }
        }
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.jpush.cn/v3/push")
        .header("Authorization", &cfg.auth.jiguang_authorization)
        .json(&json_value)
        .send()
        .await?;
    return Ok(());
}
