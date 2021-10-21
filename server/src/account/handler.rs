use crate::account::model::{PhoneAuthPostData, PhoneCodePostData};
use crate::account::service;
use crate::config::Config;
use crate::errors::ServiceError;
use crate::i18n::I18n;
use crate::middleware::auth::AuthToken;
use crate::middleware::req_meta::ReqMeta;
use crate::middleware::signature_verifier::SignatureVerifier;
use crate::types::{KvPool, Pool};
use crate::util::key_pair::Pair;
use actix_web::{web, HttpResponse};
pub async fn login_with_phone(
    phone_auth_post_data: web::Json<PhoneAuthPostData>,
    pool: web::Data<Pool>,
    i18n: web::Data<I18n>,
    req_meta: ReqMeta,
    kv: web::Data<KvPool>,
    pair: web::Data<Pair>,
    config: web::Data<Config>,
    sign: SignatureVerifier,
    text: String,
) -> Result<HttpResponse, ServiceError> {
    dbg!(sign);
    dbg!(text);
    service::login_with_phone(
        req_meta,
        phone_auth_post_data.into_inner(),
        pool.get_ref(),
        i18n.get_ref(),
        kv.get_ref(),
        pair.get_ref(),
        config.get_ref(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}
pub async fn send_phone_code(
    kv: web::Data<KvPool>,
    i18n: web::Data<I18n>,
    req_meta: ReqMeta,
    config: web::Data<Config>,
    signature: SignatureVerifier,
) -> Result<HttpResponse, ServiceError> {
    if signature.body.is_none() {
        return Err(ServiceError::BadRequest("body is empty".to_string()));
    }
    let post_data: PhoneCodePostData = serde_json::from_str(&signature.body.unwrap())?;

    service::send_phone_code(
        req_meta,
        post_data,
        kv.get_ref(),
        i18n.get_ref(),
        config.get_ref(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TestBody {
    pub test: String,
}
pub async fn test(signature: SignatureVerifier) -> Result<HttpResponse, ServiceError> {
    // dbg!(signature); // empty

    if signature.body.is_some() {
        let v: TestBody = serde_json::from_str(&signature.body.unwrap())?;
        dbg!(&v);
    }
    Ok(HttpResponse::Ok().json(""))
}
pub async fn get_user(
    kv: web::Data<KvPool>,
    i18n: web::Data<I18n>,
    req_meta: ReqMeta,
    config: web::Data<Config>,
    auth: AuthToken,
) -> Result<HttpResponse, ServiceError> {
    dbg!(auth);
    service::get_user(req_meta, kv.get_ref(), i18n.get_ref(), config.get_ref())
        .await
        .map(|res| HttpResponse::Ok().json(&res))
}
