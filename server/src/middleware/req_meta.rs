use crate::config::Config;
use accept_language::{intersection, parse};
use actix_web::dev;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorBadRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
pub const REQUEST_ID_HEADER: &str = "x-request-id";

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

#[derive(Debug, Clone)]
pub struct ReqMeta {
  pub locale: String,
}

impl ReqMeta {
  pub fn new() -> Self {
    return ReqMeta {
      locale: "zh-Hans".to_string(),
    };
  }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for ReqMeta
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = ReqMetaMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(ReqMetaMiddleware { service })
  }
}

pub struct ReqMetaMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for ReqMetaMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = S::Future;
  forward_ready!(service);
  fn call(&self, req: ServiceRequest) -> Self::Future {
    // rust newbie , bullship code
    let cfg: &web::Data<Config> = req.app_data().unwrap();
    let fallback_language = cfg.i18n.fallback_language.clone();
    let default_header_value = &HeaderValue::from_str(&fallback_language).unwrap();
    let user_accept_language = req
      .headers()
      .get("accept-language")
      .unwrap_or(default_header_value)
      .to_str()
      .ok()
      .unwrap_or("zh-Hans, en-US");
    let common_languages = intersection(
      user_accept_language,
      vec!["zh-Hans", "zh-CN", "zh", "en-US", "en"],
    );
    let mut locale = "zh-Hans";
    if common_languages.len() > 0 {
      locale = &common_languages[0];
    }
    if locale.starts_with("en") {
      locale = "en-US";
    } else {
      locale = "zh-Hans";
    }
    req.head().extensions_mut().insert(ReqMeta {
      locale: locale.to_string(),
    });
    self.service.call(req)
  }
}
impl FromRequest for ReqMeta {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  type Config = ();

  fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
    if let Some(ReqMeta { locale }) = req.extensions().get::<ReqMeta>() {
      ok(ReqMeta {
        locale: locale.clone(),
      })
    } else {
      err(ErrorBadRequest("locale is missing"))
    }
  }
}
