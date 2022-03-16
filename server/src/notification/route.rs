use crate::{
    alias::{KvPool, Pool},
    middleware::{Auth, ClientPlatform, Locale, Signature},
    notification::{
        model::{
            ApiPushParam, NotificationInbox, NotificationInboxFilter, PushForwardPayloadParam,
            PushParam, UpdateNotificationInboxParam,
        },
        service::{
            get_notification::get_notification_inbox,
            push::{push_by_account_id, push_forward},
            update_notification::update_notification_inbox,
        },
    },
    types::{JsonApiResponse, QuickResponse, ServiceResult, SimpleMetaResponse},
};

use axum::{
    extract::{Extension, Path},
    http::{header::HeaderMap, StatusCode},
    response::{Headers, IntoResponse},
    routing::{get, post},
    Json, Router,
};

pub fn service_route() -> Router {
    Router::new()
        .route(
            "/me/notification-inbox",
            get(get_me_notification_inbox_handler).patch(patch_me_notification_inbox_handler),
        )
        .route("/accounts/:account_id/push", post(push_account_handler))
}

pub async fn push_forward_handler(
    Path(registration_id): Path<String>,
    Json(payload): Json<PushForwardPayloadParam>,
) -> ServiceResult<(StatusCode, HeaderMap, String)> {
    print!("push forward");
    dbg!(&payload);
    let response = push_forward(registration_id, payload).await?;
    let status = response.status();
    // let headers_mut = response.headers_mut();
    let headers = response.headers().to_owned();
    let body = response.text().await?;
    // let body = "";
    // Ok((response.status(), body))
    Ok((status, headers, body))
}

async fn push_account_handler(
    locale: Locale,
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    auth: Auth,
    Path(account_id): Path<i64>,
    Json(payload): Json<ApiPushParam>,
) -> JsonApiResponse {
    let ApiPushParam { priority, alert } = payload;
    push_by_account_id(
        &locale,
        &pool,
        &kv,
        auth,
        PushParam {
            account_id,
            priority,
            alert,
        },
    )
    .await?;
    QuickResponse::default()
}

// async fn create_notification_handler(
//     Extension(pool): Extension<Pool>,
//     Extension(kv): Extension<KvPool>,
//     locale: Locale,
//     Json(payload): Json<CreatePotificationParam>,
//     auth: Auth,
//     Ip(ip): Ip,
// ) -> JsonApiResponse {
//     let data = create_notification(&locale, &pool, &kv, payload, auth, ip).await?;
//     Ok(Json(data.to_jsonapi_document()))
// }

async fn get_me_notification_inbox_handler(
    locale: Locale,
    _: Signature,
    Extension(pool): Extension<Pool>,
    auth: Auth,
) -> SimpleMetaResponse<NotificationInbox> {
    let data = get_notification_inbox(
        &locale,
        &pool,
        auth,
        NotificationInboxFilter {
            with_last_notification: None,
        },
    )
    .await?;
    QuickResponse::meta(data)
}

async fn patch_me_notification_inbox_handler(
    locale: Locale,
    _: Signature,
    Extension(pool): Extension<Pool>,
    auth: Auth,
    Json(payload): Json<UpdateNotificationInboxParam>,
) -> JsonApiResponse {
    let _ = update_notification_inbox(&locale, &pool, payload, auth).await?;

    QuickResponse::default()
}
