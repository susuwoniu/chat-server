use crate::{
    error::{Error, ServiceError},
    global::{Config, I18n},
    middleware::Locale,
    types::ServiceResult,
};
use once_cell::sync::OnceCell;
use reqwest::{Body, IntoUrl, Method, RequestBuilder, Response};
use std::time::Duration;
pub static XMPP_HTTP_CLIENT: OnceCell<XmppClient> = OnceCell::new();
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct XmppServerErrorResponse {
    #[serde(rename = "errCode")]
    pub error_code: i64,
    #[serde(rename = "errMsg")]
    pub error_message: String,
}
#[derive(Debug)]
pub struct XmppClient(pub reqwest::Client);
#[allow(dead_code)]
impl XmppClient {
    pub fn global() -> &'static Self {
        XMPP_HTTP_CLIENT.get().expect("read client failed")
    }
    pub fn raw_request_build<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        Self::global().0.request(method, url)
    }
    pub fn request_build(&self, method: Method, path: &str) -> RequestBuilder {
        let base_url = &Config::global().im.api_url;
        self.raw_request_build(method, format!("{}{}", base_url, path))
    }
    pub async fn format_error(response: Response) -> ServiceResult<ServiceError> {
        let body = response.json::<XmppServerErrorResponse>().await?;
        return Ok(ServiceError::internal_raw(
            &Locale::default(),
            "fetch_xmpp_service_failed",
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
            return Err(ServiceError::internal(
                &Locale::default(),
                "fetch_api_error",
                Error::Default,
            ));
        } else {
            serde_json::from_str::<T>("null").map_err(|e| e.into())
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
            .header("Content-Type", "application/json")
            .send()
            .await?;
        return Self::format_response::<T>(res).await;
    }
    pub async fn post_with_token<T: serde::de::DeserializeOwned, B: Into<Body>>(
        &self,
        path: &str,
        _: &str,
        body: B,
    ) -> ServiceResult<T> {
        let res = self
            .request_build(Method::POST, path)
            .body(body)
            .header("Content-Type", "application/json")
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
        XMPP_HTTP_CLIENT.set(XmppClient(reqwest_client)).unwrap();
    }
}
