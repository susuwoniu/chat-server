use crate::{
    file::{
        model::{CreateUploadImageSlot, UploadImageSlot},
        service::upload::create_image_upload_slot,
    },
    middleware::{Auth, Locale, Signature},
    types::{QuickResponse, SimpleMetaResponse},
};

use axum::{extract::Extension, routing::post, Json, Router};

use sonyflake::Sonyflake;

pub fn service_route() -> Router {
    Router::new().route("/image/slot", post(create_image_upload_slot_handler))
}
async fn create_image_upload_slot_handler(
    locale: Locale,
    Json(payload): Json<CreateUploadImageSlot>,
    auth: Auth,
    _: Signature,
    Extension(mut sf): Extension<Sonyflake>,
) -> SimpleMetaResponse<UploadImageSlot> {
    let data = create_image_upload_slot(&locale, payload, auth, &mut sf).await?;

    QuickResponse::meta(data)
}
