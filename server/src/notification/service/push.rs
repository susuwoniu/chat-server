use crate::{
    account::service::devices::get_devices_by_account_id,
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::config::Config,
    middleware::{Auth, Locale},
    notification::model::{AlertParam, PushForwardAlertParam, PushForwardPayloadParam, PushParam},
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

pub async fn push_forward(
    registration_id: String,
    param: PushForwardPayloadParam,
) -> ServiceResult<reqwest::Response> {
    let cfg = Config::global();
    let PushForwardPayloadParam {
        priority,
        service,
        alert,
        mode,
    } = param;
    let PushForwardAlertParam {
        title,
        body,
        badge,
        tag,
    } = alert;
    let json_value;
    let priority_str = priority.unwrap_or("normal".to_string());
    let priority_i32: i32 = match priority_str.as_str() {
        "high" => 2,
        "normal" => 0,
        _ => 0,
    };
    let tag_str = tag.unwrap_or("".to_string());
    let url;
    if tag_str != "".to_string() {
        url = format!("{}/room?id={}", cfg.application.host, tag_str);
    } else {
        url = format!("{}/", cfg.application.host);
    }

    let mut is_prod = false;

    let mode_str = mode.unwrap_or("dev".to_string());
    if mode_str == "prod" {
        is_prod = true;
    }

    let ios_level = match priority_i32 {
        2 => "active",
        0 => "critical",
        _ => "active",
    };
    if service == "fcm".to_string() {
        json_value = json!({
            "platform":["android"],
            "audience":{
                "registration_id":[registration_id]
            },
            "notification" : {
                "android":{
                    "intent":{
                        "url":url
                    },
                    "extras":{
                        "url":url
                    },
                    "alert": body,
                    "title": title.unwrap_or("".to_string()),
                    "badge_add_num": badge,
                    "priority":priority_i32
                }
            },
            "inapp_message": {"inapp_message": true}
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
                  "extras":{
                      "url":url
                  },
                "badge": badge,
                "interruption-level":ios_level,
                "thread-id":tag_str
                }
            },
            "options":{
                "apns_production":is_prod
            },
            "inapp_message": {"inapp_message": true}
        });
    } else {
        return Err(ServiceError::bad_request(
            &Locale::default(),
            "service invalid",
            Error::Default,
        ));
    }
    println!("{}", &json_value);
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
