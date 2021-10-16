use crate::account::model::PhoneLoginData;
use crate::account::service;
use crate::errors::ServiceError;
use crate::i18n::I18n;
use crate::middleware::req_meta::ReqMeta;
use crate::types::Pool;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn login_with_phone(
    account_data: web::Json<PhoneLoginData>,
    pool: web::Data<Pool>,
    i18n: web::Data<I18n>,
    req: HttpRequest,
    req_meta: ReqMeta,
) -> Result<HttpResponse, ServiceError> {
    service::login::login_with_phone(
        req,
        req_meta,
        account_data.into_inner(),
        pool.get_ref(),
        i18n.get_ref(),
    )
    .await
    .map(|res| HttpResponse::Ok().json(&res))
}
