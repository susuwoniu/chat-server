use crate::config::Client;
use crate::util::sign::Sign;
use actix_web::error::ErrorBadRequest;
use actix_web::http::header::HeaderValue;
use actix_web::web::BytesMut;
use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use futures::future::err;
use futures_util::stream::StreamExt as _;
use std::collections::HashMap;
use std::{future::Future, pin::Pin};
#[derive(Debug, Clone)]
pub struct SignatureVerifier {
  pub body: Option<String>,
}
impl FromRequest for SignatureVerifier {
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  // type Future = Ready<Result<Self, Self::Error>>;
  type Config = ();

  fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
    let client_map_ref: &web::Data<HashMap<i64, Client>> = req.app_data().unwrap();
    let client_map = client_map_ref.as_ref().clone();
    let method = req.method().to_string();
    let path = req.path().to_string();
    let query = req.query_string().to_string();
    let content_type = req
      .headers()
      .get("content-type")
      .unwrap_or(&HeaderValue::from_static("application/json"))
      .to_str()
      .unwrap()
      .to_string();
    let client_date_header_value_option = req.headers().get("x-client-date");

    if client_date_header_value_option.is_none() {
      return Box::pin(err(ErrorBadRequest(
        "missing required header x-client-date",
      )));
    }
    let client_date_header_value = client_date_header_value_option.unwrap().to_str();

    if client_date_header_value.is_err() {
      // error
      return Box::pin(err(ErrorBadRequest(
        "missing required header x-client-date",
      )));
    }
    let client_date_string = client_date_header_value.unwrap().to_string();

    let client_signature_header_value_option = req.headers().get("x-client-signature");

    if client_signature_header_value_option.is_none() {
      return Box::pin(err(ErrorBadRequest(
        "missing required header x-client-signature",
      )));
    }
    let client_signature_header_value = client_signature_header_value_option.unwrap().to_str();

    if client_signature_header_value.is_err() {
      // error
      return Box::pin(err(ErrorBadRequest(
        "missing required header x-client-signature",
      )));
    }
    let client_signature_string = client_signature_header_value.unwrap().to_string();
    let client_id_header_value_option = req.headers().get("x-client-id");

    if client_id_header_value_option.is_none() {
      return Box::pin(err(ErrorBadRequest("missing required header x-client-id")));
    }
    let client_id_header_value = client_id_header_value_option.unwrap().to_str();

    if client_id_header_value.is_err() {
      // error
      return Box::pin(err(ErrorBadRequest("missing required header x-client-id")));
    }
    let client_id_string = client_id_header_value.unwrap().to_string();
    // if client exists
    let client_id_result = client_id_string.parse::<i64>();
    if client_id_result.is_err() {
      return Box::pin(err(ErrorBadRequest("x-client-id invalid")));
    }
    let client_option = client_map.get(&client_id_result.unwrap());
    if client_option.is_none() {
      return Box::pin(err(ErrorBadRequest("client not exists")));
    }
    let client = client_option.unwrap();
    let client_secret = client.client_secret.clone();
    // parse to time
    let mut stream = payload.take();
    Box::pin(async move {
      let limit = 262_144;

      // let stream = stream.take();
      let mut body = BytesMut::with_capacity(8192);

      while let Some(item) = stream.next().await {
        let chunk = item?;
        if (body.len() + chunk.len()) > limit {
          return Err(ErrorBadRequest("body size overflow"));
        } else {
          body.extend_from_slice(&chunk);
        }
      }
      let mut raw_body = None;
      if body.len() > 0 {
        let body_string = String::from_utf8(body.to_vec()).expect("Found invalid UTF-8");
        raw_body = Some(body_string);
      }
      let sign = Sign::new(
        method,
        path,
        query,
        content_type,
        client_id_string,
        client_date_string,
        raw_body.clone(),
      );
      sign.verify(client_signature_string, client_secret)?;
      Ok(SignatureVerifier { body: raw_body })
    })
    // })
  }
}
