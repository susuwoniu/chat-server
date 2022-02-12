use crate::{
    account::{
        model::{
            AccountLikeFilter, AccountViewFilter, ApiAccountLikeFilter, ApiAccountViewFilter,
            ApiUpdateOtherAccountParam, DeviceParam, GetAccountPathParam, GetAccountsParam,
            PhoneAuthBodyParam, PhoneAuthPathParam, PhoneCodeMeta, PutImageParam,
            SendPhoneCodePathParam, SigninWithPhoneParam, UpdateAccountImageParam,
            UpdateAccountImagesParam, UpdateAccountParam, UpdateOtherAccountParam,
        },
        service::{
            get_account::{get_account_views, get_accounts, get_full_account, get_other_account},
            get_account_blocks::get_account_blocks_list,
            get_account_likes::{get_account_liked_list, get_account_likes_list},
            login_with_phone::login_with_phone,
            refresh_token_to_access_token::refresh_token_to_access_token,
            send_phone_code::send_phone_code,
            signout::signout,
            update_account::{update_account, update_other_account},
            update_account_image::{
                delete_profile_image, insert_or_update_profile_image, put_profile_images,
                update_profile_image,
            },
        },
    },
    alias::{KvPool, Pool},
    constant::ACCOUNT_SERVICE_PATH,
    file::{
        model::{CreateUploadImageSlot, UploadImageSlot},
        service::upload::create_avatar_upload_slot,
    },
    middleware::{Auth, ClientPlatform, Ip, Locale, Qs, RefreshTokenAuth, Signature},
    types::{JsonApiResponse, QuickResponse, SimpleMetaResponse},
    util::page::{format_page_links, format_page_meta, format_response},
};

use axum::{
    extract::{Extension, Path, Query},
    http::Uri,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use jsonapi::model::*;
use queue_file::QueueFile;
use sonyflake::Sonyflake;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn service_route() -> Router {
    Router::new()
        .route(
            "/phone-codes/:phone_country_code/:phone_number",
            post(send_phone_code_handler),
        )
        .route(
            "/phone-sessions/:phone_country_code/:phone_number/:code",
            post(phone_auth_handler),
        )
        .route("/sessions", delete(signout_handler))
        .route(
            "/accounts/:account_id",
            get(get_account_handler).patch(patch_other_account_handler),
        )
        .route("/admin/accounts/:account_id", patch(patch_account_handler))
        .route("/accounts", get(get_accounts_by_ids_handler))
        .route("/me", get(get_me_handler).patch(patch_me_account_handler))
        .route("/me/profile-images", put(put_me_images_handler))
        .route(
            "/me/profile-images/:order",
            put(put_me_image_handler)
                .delete(delete_me_profile_image)
                .patch(patch_me_image_handler),
        )
        .route("/me/views", get(get_me_views_handler))
        .route("/me/liked", get(get_me_liked_list_handler))
        .route("/me/likes", get(get_me_likes_list_handler))
        .route("/me/blocks", get(get_me_blocks_list_handler))
        .route("/me/avatar/slot", post(create_avatar_upload_slot_handler))
        .route("/access-tokens", post(access_token_handler))
}
async fn delete_me_profile_image(
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    locale: Locale,
    Path(order): Path<u16>,
    auth: Auth,
) -> JsonApiResponse {
    let data = delete_profile_image(&locale, &pool, &kv, &auth, order as i16).await?;
    Ok(format_response(data.to_jsonapi_document()))
}
async fn patch_me_image_handler(
    Path(order): Path<u16>,
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    locale: Locale,
    auth: Auth,
    Json(payload): Json<PutImageParam>,
    _: Signature,
) -> JsonApiResponse {
    let data = update_profile_image(
        &locale,
        &pool,
        &kv,
        &auth,
        UpdateAccountImageParam {
            order: order as i16,
            url: payload.url,
            width: payload.width,
            height: payload.height,
            size: payload.size,
            mime_type: payload.mime_type,
        },
    )
    .await?;
    Ok(format_response(data.to_jsonapi_document()))
}
async fn put_me_image_handler(
    Path(order): Path<u16>,
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,

    locale: Locale,
    auth: Auth,
    Json(payload): Json<PutImageParam>,
    _: Signature,
    Extension(mut sf): Extension<Sonyflake>,
) -> JsonApiResponse {
    let data = insert_or_update_profile_image(
        &locale,
        &pool,
        &kv,
        &auth,
        UpdateAccountImageParam {
            order: order as i16,
            url: payload.url,
            width: payload.width,
            height: payload.height,
            size: payload.size,
            mime_type: payload.mime_type,
        },
        &mut sf,
    )
    .await?;
    Ok(format_response(data.to_jsonapi_document()))
}
async fn put_me_images_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    Extension(kv): Extension<KvPool>,
    auth: Auth,
    Json(payload): Json<UpdateAccountImagesParam>,
    _: Signature,
    Extension(mut sf): Extension<Sonyflake>,
) -> JsonApiResponse {
    let data = put_profile_images(&locale, &pool, &kv, payload, &mut sf, &auth).await?;
    Ok(format_response(data.to_jsonapi_document()))
}

async fn get_me_views_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    Qs(filter): Qs<ApiAccountViewFilter>,
    Query(query): Query<HashMap<String, String>>,
    uri: Uri,
    auth: Auth,
) -> JsonApiResponse {
    let account_view_filter = AccountViewFilter::try_from(filter)?;
    let data = get_account_views(&locale, &pool, &account_view_filter, auth.account_id).await?;
    let resources = vec_to_jsonapi_resources(data.data);
    let json_api_data = resources.0;
    let other = resources.1;
    let response = JsonApiDocument::Data(DocumentData {
        meta: Some(format_page_meta(data.page_info.clone())),
        data: Some(PrimaryData::Multiple(json_api_data)),
        links: Some(format_page_links(
            ACCOUNT_SERVICE_PATH,
            uri.path(),
            query,
            data.page_info,
        )),
        included: other,
        ..Default::default()
    });
    Ok(format_response(response))
}

async fn get_me_liked_list_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    Qs(filter): Qs<ApiAccountLikeFilter>,
    Query(query): Query<HashMap<String, String>>,
    uri: Uri,
    auth: Auth,
) -> JsonApiResponse {
    let account_view_filter = AccountLikeFilter::try_from(filter)?;
    let data =
        get_account_liked_list(&locale, &pool, &account_view_filter, auth.account_id).await?;
    let resources = vec_to_jsonapi_resources(data.data);
    let json_api_data = resources.0;
    let other = resources.1;
    let response = JsonApiDocument::Data(DocumentData {
        meta: Some(format_page_meta(data.page_info.clone())),
        data: Some(PrimaryData::Multiple(json_api_data)),
        links: Some(format_page_links(
            ACCOUNT_SERVICE_PATH,
            uri.path(),
            query,
            data.page_info,
        )),
        included: other,
        ..Default::default()
    });
    Ok(format_response(response))
}
async fn get_me_likes_list_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    Qs(filter): Qs<ApiAccountLikeFilter>,
    Query(query): Query<HashMap<String, String>>,
    uri: Uri,
    auth: Auth,
) -> JsonApiResponse {
    let account_view_filter = AccountLikeFilter::try_from(filter)?;
    let data =
        get_account_likes_list(&locale, &pool, &account_view_filter, auth.account_id).await?;
    let resources = vec_to_jsonapi_resources(data.data);
    let json_api_data = resources.0;
    let other = resources.1;
    let response = JsonApiDocument::Data(DocumentData {
        meta: Some(format_page_meta(data.page_info.clone())),
        data: Some(PrimaryData::Multiple(json_api_data)),
        links: Some(format_page_links(
            ACCOUNT_SERVICE_PATH,
            uri.path(),
            query,
            data.page_info,
        )),
        included: other,
        ..Default::default()
    });
    Ok(format_response(response))
}

async fn get_me_blocks_list_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    Qs(filter): Qs<ApiAccountLikeFilter>,
    Query(query): Query<HashMap<String, String>>,
    uri: Uri,
    auth: Auth,
) -> JsonApiResponse {
    let account_view_filter = AccountLikeFilter::try_from(filter)?;
    let data =
        get_account_blocks_list(&locale, &pool, &account_view_filter, auth.account_id).await?;
    let resources = vec_to_jsonapi_resources(data.data);
    let json_api_data = resources.0;
    let other = resources.1;
    let response = JsonApiDocument::Data(DocumentData {
        meta: Some(format_page_meta(data.page_info.clone())),
        data: Some(PrimaryData::Multiple(json_api_data)),
        links: Some(format_page_links(
            ACCOUNT_SERVICE_PATH,
            uri.path(),
            query,
            data.page_info,
        )),
        included: other,
        ..Default::default()
    });
    Ok(format_response(response))
}

async fn patch_other_account_handler(
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    Path(target_accoutn_id): Path<i64>,
    locale: Locale,
    auth: Auth,
    Json(payload): Json<ApiUpdateOtherAccountParam>,
    Extension(mut sf): Extension<Sonyflake>,
) -> JsonApiResponse {
    update_other_account(
        &locale,
        &pool,
        &kv,
        UpdateOtherAccountParam {
            viewed_count_action: payload.viewed_count_action,
            target_account_id: target_accoutn_id,
            like_count_action: payload.like_count_action,
            block_count_action: payload.block_count_action,
        },
        auth,
        &mut sf,
    )
    .await?;
    QuickResponse::default()
}
async fn patch_account_handler(
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    Path(target_accoutn_id): Path<i64>,
    locale: Locale,
    auth: Auth,
    Json(mut payload): Json<UpdateAccountParam>,
) -> JsonApiResponse {
    payload.account_id = Some(target_accoutn_id);
    let account = update_account(&locale, &pool, &kv, payload, &auth, false).await?;
    Ok(format_response(account.to_jsonapi_document()))
}
async fn patch_me_account_handler(
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,

    locale: Locale,
    auth: Auth,
    Json(mut payload): Json<UpdateAccountParam>,
) -> JsonApiResponse {
    payload.account_id = Some(auth.account_id);
    let account = update_account(&locale, &pool, &kv, payload, &auth, false).await?;
    Ok(format_response(account.to_jsonapi_document()))
}
async fn signout_handler(Extension(kv): Extension<KvPool>, auth: Auth) -> JsonApiResponse {
    signout(&kv, &auth).await?;
    QuickResponse::default()
}
async fn access_token_handler(
    Extension(pool): Extension<Pool>,

    Extension(kv): Extension<KvPool>,
    locale: Locale,
    auth: RefreshTokenAuth,
    platform: ClientPlatform,
    Ip(ip): Ip,
    Extension(mut sf): Extension<Sonyflake>,
) -> JsonApiResponse {
    let data =
        refresh_token_to_access_token(&locale, &pool, &kv, &auth, ip, platform, &mut sf).await?;
    Ok(format_response(data.to_jsonapi_document()))
}

async fn phone_auth_handler(
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    locale: Locale,
    Path(path_param): Path<PhoneAuthPathParam>,
    Signature { client_id }: Signature,
    Json(payload): Json<PhoneAuthBodyParam>,
    Ip(ip): Ip,
    platform: ClientPlatform,
    Extension(mut sf): Extension<Sonyflake>,
    Extension(qf_mutex): Extension<Arc<Mutex<QueueFile>>>,
) -> JsonApiResponse {
    let PhoneAuthPathParam {
        phone_country_code,
        phone_number,
        code,
    } = path_param;

    let auth_data = login_with_phone(
        &locale,
        &pool,
        &kv,
        &SigninWithPhoneParam {
            phone_country_code,
            phone_number,
            code,
            client_id,
            device_id: payload.device_id,
            timezone_in_seconds: payload.timezone_in_seconds,
            ip,
            platform,
            qf_mutex,
        },
        &mut sf,
    )
    .await?;
    let doc = auth_data.to_jsonapi_document();
    Ok(format_response(doc))
}

async fn get_account_handler(
    Extension(pool): Extension<Pool>,
    Path(path_param): Path<GetAccountPathParam>,
    locale: Locale,
    auth: Option<Auth>,
) -> JsonApiResponse {
    let account = get_other_account(&locale, &pool, path_param.account_id, auth).await?;
    Ok(format_response(account.to_jsonapi_document()))
}
async fn get_accounts_by_ids_handler(
    Extension(pool): Extension<Pool>,
    Qs(query): Qs<GetAccountsParam>,
    locale: Locale,
) -> JsonApiResponse {
    let data = get_accounts(&locale, &pool, query.ids).await?;
    Ok(format_response(vec_to_jsonapi_document(data)))
}
async fn get_me_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    auth: Auth,
) -> JsonApiResponse {
    let account = get_full_account(&locale, &pool, auth.account_id).await?;
    let doc = account.to_jsonapi_document();

    Ok(format_response(doc))
}
async fn send_phone_code_handler(
    Path(path_param): Path<SendPhoneCodePathParam>,
    Extension(kv): Extension<KvPool>,
    locale: Locale,
    Json(payload): Json<DeviceParam>,
    _: Signature,
) -> SimpleMetaResponse<PhoneCodeMeta> {
    let data = send_phone_code(&locale, &kv, path_param, payload).await?;
    QuickResponse::meta(data)
}

async fn create_avatar_upload_slot_handler(
    locale: Locale,
    Json(payload): Json<CreateUploadImageSlot>,
    auth: Auth,
    Extension(mut sf): Extension<Sonyflake>,
    _: Signature,
) -> SimpleMetaResponse<UploadImageSlot> {
    let data = create_avatar_upload_slot(&locale, payload, auth, &mut sf).await?;

    QuickResponse::meta(data)
}
