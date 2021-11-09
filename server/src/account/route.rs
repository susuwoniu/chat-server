use crate::{
  account::{
    model::{
      AddImageParam, DeviceParam, GetAccountPathParam, PhoneAuthBodyParam, PhoneAuthPathParam,
      PhoneCodeMeta, SendPhoneCodePathParam, SigninWithPhoneParam, UpdateAccountImageParam,
      UpdateAccountParam,
    },
    service::{
      get_account::{get_account, get_slim_account},
      login_with_phone::login_with_phone,
      refresh_token_to_access_token::refresh_token_to_access_token,
      send_phone_code::send_phone_code,
      signout::signout,
      update_account::update_account,
      update_account_image::{
        delete_profile_image, get_profile_images, insert_profile_image, update_profile_image,
      },
    },
  },
  alias::{KvPool, Pool},
  middleware::{Auth, ClientVersion, Ip, Locale, RefreshTokenAuth, Signature},
  types::{JsonApiResponse, QuickResponse, SimpleMetaResponse},
};

use axum::{
  extract::{Extension, Path},
  routing::{delete, get, post},
  Json, Router,
};
use jsonapi::{api::*, model::*};

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
    .route("/accounts/:account_id", get(get_account_handler))
    .route("/me", get(get_me_handler).patch(patch_account_handler))
    .route(
      "/me/profile-images/:order",
      post(add_me_image_handler)
        .patch(patch_me_image_handler)
        .delete(delete_me_profile_image),
    )
    .route(
      "/me/profile-images",
      get(get_me_images_handler).patch(patch_account_handler),
    )
    .route("/access-tokens", post(access_token_handler))
}
async fn delete_me_profile_image(
  Extension(pool): Extension<Pool>,
  Path(sequence): Path<u32>,
  Auth { account_id, .. }: Auth,
) -> JsonApiResponse {
  delete_profile_image(&pool, &account_id, sequence as i32).await?;
  QuickResponse::default()
}
async fn patch_me_image_handler(
  Path(sequence): Path<u32>,
  Extension(pool): Extension<Pool>,
  locale: Locale,
  auth: Auth,
  Json(payload): Json<AddImageParam>,
  _: Signature,
) -> JsonApiResponse {
  let data = update_profile_image(
    &locale,
    &pool,
    &auth.account_id,
    UpdateAccountImageParam {
      sequence: sequence as i32,
      url: payload.url,
    },
  )
  .await?;
  let (res, _) = data.to_jsonapi_resource();
  let doc = JsonApiDocument::Data(DocumentData {
    data: Some(PrimaryData::Single(Box::new(res))),
    ..Default::default()
  });
  Ok(Json(doc))
}
async fn add_me_image_handler(
  Path(sequence): Path<u32>,
  Extension(pool): Extension<Pool>,
  locale: Locale,
  auth: Auth,
  Json(payload): Json<AddImageParam>,
  _: Signature,
) -> JsonApiResponse {
  let data = insert_profile_image(
    &locale,
    &pool,
    &auth.account_id,
    UpdateAccountImageParam {
      sequence: sequence as i32,
      url: payload.url,
    },
  )
  .await?;
  let (res, _) = data.to_jsonapi_resource();
  let doc = JsonApiDocument::Data(DocumentData {
    data: Some(PrimaryData::Single(Box::new(res))),
    ..Default::default()
  });
  Ok(Json(doc))
}

async fn get_me_images_handler(Extension(pool): Extension<Pool>, auth: Auth) -> JsonApiResponse {
  let data = get_profile_images(&pool, &auth.account_id).await?;

  Ok(Json(vec_to_jsonapi_document(data)))
}
async fn patch_account_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  auth: Auth,
  Json(payload): Json<UpdateAccountParam>,
) -> JsonApiResponse {
  update_account(&locale, &pool, &auth.account_id, payload, &auth).await?;
  QuickResponse::default()
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
  Ip(ip): Ip,
) -> JsonApiResponse {
  let data = refresh_token_to_access_token(&locale, &pool, &kv, &auth, ip).await?;
  Ok(Json(data.to_jsonapi_document()))
}

async fn phone_auth_handler(
  Extension(pool): Extension<Pool>,
  Extension(kv): Extension<KvPool>,
  locale: Locale,
  Path(path_param): Path<PhoneAuthPathParam>,
  Signature { client_id }: Signature,
  Json(payload): Json<PhoneAuthBodyParam>,
  Ip(ip): Ip,
  ClientVersion(version): ClientVersion,
) -> JsonApiResponse {
  dbg!(&version);
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
    },
  )
  .await?;
  let doc = auth_data.to_jsonapi_document();
  Ok(Json(doc))
}

async fn get_account_handler(
  Extension(pool): Extension<Pool>,
  Path(path_param): Path<GetAccountPathParam>,
  locale: Locale,
) -> JsonApiResponse {
  let account = get_slim_account(&locale, &pool, &path_param.account_id).await?;
  Ok(Json(account.to_jsonapi_document()))
}
async fn get_me_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  auth: Auth,
) -> JsonApiResponse {
  let account = get_account(&locale, &pool, &auth.account_id).await?;
  let doc = account.to_jsonapi_document();
  Ok(Json(doc))
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
