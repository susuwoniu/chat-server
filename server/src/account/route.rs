use crate::{
  account::{
    model::{
      Account, DeviceParam, GetAccountPathParam, PhoneAuthBodyParam, PhoneAuthPathParam,
      PhoneCodeResponseData, SendPhoneCodePathParam, SigninWithPhoneParam, SlimAccount,
      SuccessResponseData, UpdateAccountParam,
    },
    service::{
      get_account::{get_account, get_slim_account},
      login_with_phone::login_with_phone,
      refresh_token_to_access_token::refresh_token_to_access_token,
      send_phone_code::send_phone_code,
      signout::signout,
      update_account::update_account,
    },
    util::AuthData,
  },
  alias::{KvPool, Pool},
  error::ServiceJson,
  middleware::{Auth, Locale, RefreshTokenAuth, Signature},
};
use axum::{
  extract::{Extension, Path},
  routing::{delete, get, patch, post},
  Json, Router,
};

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
    .route("/access-tokens", post(access_token_handler))
}
async fn patch_account_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  Auth { account_id, .. }: Auth,
  Json(payload): Json<UpdateAccountParam>,
) -> ServiceJson<SuccessResponseData> {
  Ok(Json(
    update_account(&locale, &pool, &account_id, payload).await?,
  ))
}
async fn signout_handler(
  Extension(kv): Extension<KvPool>,
  locale: Locale,
  auth: Auth,
) -> ServiceJson<SuccessResponseData> {
  Ok(Json(signout(&locale, &kv, &auth).await?))
}
async fn access_token_handler(
  Extension(pool): Extension<Pool>,

  Extension(kv): Extension<KvPool>,
  locale: Locale,
  auth: RefreshTokenAuth,
) -> ServiceJson<AuthData> {
  Ok(Json(
    refresh_token_to_access_token(&locale, &pool, &kv, &auth).await?,
  ))
}

async fn phone_auth_handler(
  Extension(pool): Extension<Pool>,
  Extension(kv): Extension<KvPool>,
  locale: Locale,
  Path(path_param): Path<PhoneAuthPathParam>,
  Signature { client_id }: Signature,
  Json(payload): Json<PhoneAuthBodyParam>,
) -> ServiceJson<AuthData> {
  let PhoneAuthPathParam {
    phone_country_code,
    phone_number,
    code,
  } = path_param;
  Ok(Json(
    login_with_phone(
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
      },
    )
    .await?,
  ))
}

async fn get_account_handler(
  Extension(pool): Extension<Pool>,
  Path(path_param): Path<GetAccountPathParam>,
  locale: Locale,
) -> ServiceJson<SlimAccount> {
  Ok(Json(
    get_slim_account(&locale, &pool, &path_param.account_id).await?,
  ))
}
async fn get_me_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  auth: Auth,
) -> ServiceJson<Account> {
  dbg!(&auth);
  Ok(Json(get_account(&locale, &pool, &auth.account_id).await?))
}
async fn send_phone_code_handler(
  Path(path_param): Path<SendPhoneCodePathParam>,
  Extension(kv): Extension<KvPool>,
  locale: Locale,
  Json(payload): Json<DeviceParam>,
  _: Signature,
) -> ServiceJson<PhoneCodeResponseData> {
  Ok(Json(
    send_phone_code(&locale, &kv, path_param, payload).await?,
  ))
}
