use crate::{
    alias::{KvPool, Pool},
    middleware::{Auth, Ip, Locale, Qs, Signature},
    notification::{
        model::{
            DbNotification, DbNotificationInbox, NotificationInbox, NotificationInboxFilter,
            NotificationInboxItem, NotificationType, UpdateNotificationInboxParam,
        },
        service::{
            get_notification::get_notification_inboxes,
            update_notification::update_notification_inbox,
        },
    },
    types::{JsonApiResponse, QuickResponse, SimpleMetaResponse},
    util::page::{format_page_links, format_page_meta},
};

use axum::{
    extract::{Extension, Path, Query},
    http::Uri,
    routing::get,
    Json, Router,
};
use jsonapi::{api::*, model::*};

pub fn service_route() -> Router {
    Router::new().route(
        "/me/notification_inboxes",
        get(get_me_notification_inboxes_handler).patch(patch_me_notification_inboxes_handler),
    )
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

async fn get_me_notification_inboxes_handler(
    locale: Locale,
    _: Signature,
    Extension(pool): Extension<Pool>,
    auth: Auth,
) -> SimpleMetaResponse<NotificationInbox> {
    let data = get_notification_inboxes(
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

async fn patch_me_notification_inboxes_handler(
    locale: Locale,
    _: Signature,
    Extension(pool): Extension<Pool>,
    auth: Auth,
    Json(payload): Json<UpdateNotificationInboxParam>,
) -> JsonApiResponse {
    let _ = update_notification_inbox(&locale, &pool, payload, auth).await?;

    QuickResponse::default()
}
