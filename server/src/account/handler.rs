use crate::account::model::{PhoneAuthPostData, PhoneCodePostData};
use crate::account::service;
use crate::error::{Error, ServiceError};
use crate::middleware::auth::AuthToken;
use crate::middleware::req_meta::ReqMeta;
use crate::middleware::signature_verifier::SignatureVerifier;
use crate::types::{KvPool, Pool};
use crate::util::key_pair::Pair;
use actix_web::{web, HttpResponse};
pub async fn login_with_phone(
    pool: web::Data<Pool>,
    req_meta: ReqMeta,
    kv: web::Data<KvPool>,
    pair: web::Data<Pair>,
    signature: SignatureVerifier,
) -> Result<HttpResponse, ServiceError> {
    if signature.body.is_none() {
        return Err(ServiceError::bad_request(
            &req_meta.locale,
            "body_can_not_be_empty",
            Error::Other("post body is empty login_with_phone".to_string()),
        ));
    }
    let post_data: PhoneAuthPostData = serde_json::from_str(&signature.body.unwrap())?;

    service::login_with_phone(
        req_meta,
        post_data,
        pool.get_ref(),
        kv.get_ref(),
        pair.get_ref(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}
pub async fn send_phone_code(
    kv: web::Data<KvPool>,
    req_meta: ReqMeta,
    signature: SignatureVerifier,
) -> Result<HttpResponse, ServiceError> {
    if signature.body.is_none() {
        return Err(ServiceError::bad_request(
            &req_meta.locale,
            "body_can_not_be_empty",
            Error::Other("post body is empty login_with_phone".to_string()),
        ));
    }
    let post_data: PhoneCodePostData = serde_json::from_str(&signature.body.unwrap())?;

    service::send_phone_code(req_meta, post_data, kv.get_ref())
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
#[derive(Deserialize)]
pub struct SingleSlimAccountPath {
    account_id: i64,
}
pub async fn get_slim_account(
    pool: web::Data<Pool>,
    req_meta: ReqMeta,
    info: web::Path<SingleSlimAccountPath>,
) -> Result<HttpResponse, ServiceError> {
    service::get_slim_user(pool.as_ref(), info.account_id, req_meta)
        .await
        .map(|res| HttpResponse::Ok().json(&res))
}

pub async fn get_me(
    pool: web::Data<Pool>,
    req_meta: ReqMeta,
    auth: AuthToken,
) -> Result<HttpResponse, ServiceError> {
    service::get_user(pool.as_ref(), auth.account_id, req_meta)
        .await
        .map(|res| HttpResponse::Ok().json(&res))
}
