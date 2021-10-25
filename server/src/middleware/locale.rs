use crate::{
  error::ServiceError, global::Config, middleware::header_accept_language::AcceptLangauge,
};
use accept_language::intersection;
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts, TypedHeader},
};
pub struct Locale(pub String);
impl Locale {
  pub fn new(lang: &str) -> Self {
    Locale(lang.to_string())
  }
}
impl Default for Locale {
  fn default() -> Self {
    Self(Config::global().i18n.fallback_language.clone())
  }
}
#[async_trait]
impl<B> FromRequest<B> for Locale
where
  B: Send,
{
  type Rejection = ServiceError;
  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let cfg = Config::global();
    let accept_langauge_result = TypedHeader::<AcceptLangauge>::from_request(req).await;
    let mut accept_language_string: String = cfg.i18n.fallback_language.clone();
    if let Ok(accept_langauge) = accept_langauge_result {
      // parse language
      accept_language_string = accept_langauge.0 .0;
    }
    let common_languages = intersection(
      &accept_language_string,
      vec!["zh-Hans", "zh-CN", "zh", "en-US", "en"],
    );
    let mut locale_string: &str = &cfg.i18n.fallback_language;
    if common_languages.len() > 0 {
      locale_string = &common_languages[0];
    }
    if locale_string.starts_with("en") {
      locale_string = "en-US";
    } else {
      locale_string = "zh-Hans";
    }

    Ok(Locale(locale_string.to_string()))
  }
}
