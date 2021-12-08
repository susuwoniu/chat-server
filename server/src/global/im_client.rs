use crate::{
  error::{Error, ServiceError},
  global::{Config, I18n},
  middleware::Locale,
  types::ServiceResult,
};
use once_cell::sync::OnceCell;
use reqwest::{Body, IntoUrl, Method, RequestBuilder, Response};
use std::{collections::HashMap, time::Duration};
pub static HTTP_CLIENT: OnceCell<ImClient> = OnceCell::new();
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
#[derive(Debug, Serialize, Deserialize)]
pub struct ImServerErrorResponse {
  #[serde(rename = "errCode")]
  pub error_code: i64,
  #[serde(rename = "errMsg")]
  pub error_message: String,
}
#[derive(Debug)]
pub struct ImClient(pub reqwest::Client);
#[allow(dead_code)]
impl ImClient {
  pub fn global() -> &'static Self {
    HTTP_CLIENT.get().expect("read client failed")
  }
  pub fn raw_request_build<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
    Self::global().0.request(method, url)
  }
  pub fn request_build(&self, method: Method, path: &str) -> RequestBuilder {
    let base_url = &Config::global().im.api_url;
    self.raw_request_build(method, format!("{}{}", base_url, path))
  }
  pub async fn format_error(response: Response) -> ServiceResult<ServiceError> {
    let body = response.json::<ImServerErrorResponse>().await?;
    return Ok(ServiceError::internal_raw(
      &Locale::default(),
      "fetch_im_service_failed",
      &I18n::global().get("internal-error-title", &Locale::default()),
      Some(&body.error_message),
      Error::Default,
    ));
  }
  pub async fn format_response<T: serde::de::DeserializeOwned>(
    response: Response,
  ) -> ServiceResult<T> {
    let status = response.status();

    if !status.is_success() {
      return Err(Self::format_error(response).await?);
    } else {
      // check errCode
      let body_string = response.text().await?;
      let body: HashMap<String, Value> = serde_json::from_str(&body_string)?;
      if let Some(error_code) = body.get("errCode") {
        if error_code != &json!(0) {
          let error_message = body.get("errMsg");
          if let Some(error_message) = error_message {
            return Err(ServiceError::internal_raw(
              &Locale::default(),
              "fetch_im_service_failed",
              &I18n::global().get("internal-error-title", &Locale::default()),
              Some(error_message.as_str().unwrap_or("")),
              Error::Default,
            ));
          } else {
            return Err(ServiceError::internal_raw(
              &Locale::default(),
              "fetch_im_service_response_parse_failed",
              &I18n::global().get("internal-error-title", &Locale::default()),
              Some(&I18n::global().get("internal-error-detail", &Locale::default())),
              Error::Default,
            ));
          }
        } else {
          serde_json::from_str::<T>(&body_string).map_err(|e| e.into())
        }
      } else {
        return Err(ServiceError::internal_raw(
          &Locale::default(),
          "fetch_im_service_response_parse_failed",
          &I18n::global().get("internal-error-title", &Locale::default()),
          Some(&I18n::global().get("internal-error-detail", &Locale::default())),
          Error::Default,
        ));
      }
    }
  }
  pub async fn post<T: serde::de::DeserializeOwned, B: Into<Body>>(
    &self,
    path: &str,
    body: B,
  ) -> ServiceResult<T> {
    let res = self
      .request_build(Method::POST, path)
      .body(body)
      .send()
      .await?;
    return Self::format_response::<T>(res).await;
  }
  pub async fn post_with_token<T: serde::de::DeserializeOwned, B: Into<Body>>(
    &self,
    path: &str,
    token: &str,
    body: B,
  ) -> ServiceResult<T> {
    let res = self
      .request_build(Method::POST, path)
      .body(body)
      .header("token", token)
      .send()
      .await?;
    return Self::format_response::<T>(res).await;
  }
  pub fn init() {
    let reqwest_client = reqwest::Client::builder()
      .connect_timeout(Duration::from_secs(10))
      .timeout(Duration::from_secs(30))
      .pool_max_idle_per_host(1)
      .build()
      .unwrap();
    HTTP_CLIENT.set(ImClient(reqwest_client)).unwrap();
  }
}
