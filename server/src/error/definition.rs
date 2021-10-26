use super::Error;
use super::ServiceError;
use crate::{global::I18n, middleware::Locale};
use axum::http::StatusCode;
use chrono::NaiveDateTime;
use fluent_bundle::FluentArgs;
impl ServiceError {
  pub fn new(
    locale: &Locale,
    status: StatusCode,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    tracing::error!(
      "Error occured: code:{}, title: {},detail: {:?}, stack:{:?}",
      code,
      title,
      detail,
      stack
    );
    let mut final_detail: String = String::from("");
    if detail.is_some() {
      let the_detail_string = detail.unwrap();
      final_detail.push_str(the_detail_string);
    }
    let mut args = FluentArgs::new();
    args.set("code", code);
    let code_info = I18n::global().with_args("error-code-detail", locale, args);
    final_detail.push_str(" ");
    final_detail.push_str(&code_info);
    return ServiceError {
      status,
      code: code.to_string(),
      title: title.to_string(),
      detail: final_detail.to_string(),
    };
  }
  pub fn forbidden_raw(
    locale: &Locale,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(locale, StatusCode::FORBIDDEN, code, title, detail, stack)
  }
  pub fn bad_request_raw(
    locale: &Locale,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(locale, StatusCode::BAD_REQUEST, code, title, detail, stack)
  }
  pub fn not_found_raw(
    locale: &Locale,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(locale, StatusCode::NOT_FOUND, code, title, detail, stack)
  }
  pub fn internal_raw(
    locale: &Locale,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(
      locale,
      StatusCode::INTERNAL_SERVER_ERROR,
      code,
      title,
      detail,
      stack,
    )
  }
  pub fn unauthorized_raw(
    locale: &Locale,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(locale, StatusCode::UNAUTHORIZED, code, title, detail, stack)
  }
  pub fn too_many_requests_raw(
    locale: &Locale,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(
      locale,
      StatusCode::TOO_MANY_REQUESTS,
      code,
      title,
      detail,
      stack,
    )
  }
  pub fn internal(locale: &Locale, code: &str, stack: Error) -> Self {
    Self::internal_raw(
      locale,
      code,
      &I18n::global().get("internal-error-title", locale),
      Some(&I18n::global().get("internal-error-detail", locale)),
      stack,
    )
  }
  pub fn bad_request(locale: &Locale, code: &str, stack: Error) -> Self {
    Self::bad_request_raw(
      locale,
      code,
      &I18n::global().get("bad-request-title", locale),
      Some(&I18n::global().get("bad-request-detail", locale)),
      stack,
    )
  }
  pub fn unauthorized(locale: &Locale, code: &str, stack: Error) -> Self {
    Self::unauthorized_raw(
      locale,
      code,
      &I18n::global().get("unauthorized-title", locale),
      Some(&I18n::global().get("unauthorized-detail", locale)),
      stack,
    )
  }
  pub fn phone_code_expired(locale: &Locale, stack: Error) -> Self {
    let code = "phone_code_expired";
    Self::bad_request_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn phone_code_failed_or_expired(locale: &Locale, stack: Error) -> Self {
    let code = "phone_code_expired";
    Self::bad_request_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn get_phone_code_too_many_requests(locale: &Locale, stack: Error) -> Self {
    let code = "get_phone_code_too_many_requests";
    Self::too_many_requests_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn account_suspended(
    locale: &Locale,
    reason: Option<String>,
    suspended_until: Option<NaiveDateTime>,
    stack: Error,
  ) -> Self {
    let code = "account_suspended";
    let mut args = FluentArgs::new();
    args.set(
      "reason",
      reason.unwrap_or(I18n::global().get("account-suspended-default-reason", locale)),
    );
    let mut suspended_until_final = I18n::global().get("account-suspended-default-until", locale);

    if let Some(suspend_until_naive_time) = suspended_until {
      suspended_until_final = suspend_until_naive_time.to_string();
    }
    args.set("suspended_until", suspended_until_final);
    Self::bad_request_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().with_args(&get_detail(code), locale, args)),
      stack,
    )
  }
  pub fn account_not_exist(locale: &Locale, stack: Error) -> Self {
    let code = "account_not_exist";
    Self::not_found_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
  #[allow(dead_code)]
  pub fn base_bad_request() -> Self {
    let code = "base_bad_request";
    Self::not_found_raw(
      &Locale::default(),
      code,
      &I18n::global().get(&get_title(code), &Locale::default()),
      Some(&I18n::global().get(&get_detail(code), &Locale::default())),
      Error::Other(format!("base bad request")),
    )
  }
  pub fn client_id_not_exist(locale: &Locale, stack: Error) -> Self {
    let code = "client_id_not_exist";
    Self::not_found_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn account_age_invalid(locale: &Locale, stack: Error) -> Self {
    let code = "account_age_invalid";
    Self::bad_request_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn permission_limit(locale: &Locale, stack: Error) -> Self {
    let code = "permission_limit";
    Self::forbidden_raw(
      locale,
      code,
      &I18n::global().get(&get_title(code), locale),
      Some(&I18n::global().get(&get_detail(code), locale)),
      stack,
    )
  }
}

fn get_title(code: &str) -> String {
  return format!("{}-title", str::replace(code, "_", "-"));
}
fn get_detail(code: &str) -> String {
  return format!("{}-detail", str::replace(code, "_", "-"));
}
