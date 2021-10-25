use crate::{
  account::{
    model::{
      Account, GetAccountPathParam, PhoneAuthBodyParam, PhoneAuthPathParam, PhoneCodeResponseData,
      SendPhoneCodePathParam, SigninWithPhoneParam, SlimAccount,
    },
    service::{
      get_account::{get_account, get_slim_account},
      login_with_phone::login_with_phone,
      send_phone_code::send_phone_code,
    },
    util::AuthData,
  },
  alias::{KvPool, Pool},
  error::ServiceJson,
  middleware::{Auth, Locale, Signature},
};
use axum::{
  extract::{Extension, Path},
  routing::{get, post},
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
    .route("/accounts/:account_id", get(get_account_handler))
    .route("/me", get(get_me_handler))
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
    get_slim_account(&pool, &path_param.account_id, &locale).await?,
  ))
}
async fn get_me_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  auth: Auth,
) -> ServiceJson<Account> {
  Ok(Json(get_account(&pool, &auth.account_id, &locale).await?))
}
async fn send_phone_code_handler(
  Path(path_param): Path<SendPhoneCodePathParam>,
  Extension(kv): Extension<KvPool>,
  locale: Locale,
  _: Signature,
) -> ServiceJson<PhoneCodeResponseData> {
  Ok(Json(send_phone_code(path_param, &kv, &locale).await?))
}
