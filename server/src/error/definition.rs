use super::Error;
use super::ServiceError;
use crate::i18n::I18N;
use actix_web::http::StatusCode;
use fluent_bundle::FluentArgs;
impl ServiceError {
  pub fn new(
    locale: &str,
    status: u16,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    error!(
      "Error occured: code:{}, title: {},detail: {:?}, stack:{:?}",
      code, title, detail, stack
    );
    let mut final_detail: String = String::from("");
    if detail.is_some() {
      let the_detail_string = detail.unwrap();
      final_detail.push_str(the_detail_string);
    }
    let mut args = FluentArgs::new();
    args.set("code", code);
    let code_info = &I18N
      .read()
      .unwrap()
      .with_args("error-code-detail", locale, args);
    final_detail.push_str(code_info);
    return ServiceError {
      status,
      code: code.to_string(),
      title: title.to_string(),
      detail: final_detail.to_string(),
    };
  }

  pub fn bad_request_raw(
    locale: &str,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(
      locale,
      StatusCode::BAD_REQUEST.as_u16(),
      code,
      title,
      detail,
      stack,
    )
  }
  pub fn internal_raw(
    locale: &str,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(
      locale,
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
      code,
      title,
      detail,
      stack,
    )
  }
  pub fn unauthorized_raw(
    locale: &str,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(
      locale,
      StatusCode::UNAUTHORIZED.as_u16(),
      code,
      title,
      detail,
      stack,
    )
  }
  pub fn too_many_requests_raw(
    locale: &str,
    code: &str,
    title: &str,
    detail: Option<&str>,
    stack: Error,
  ) -> Self {
    Self::new(
      locale,
      StatusCode::TOO_MANY_REQUESTS.as_u16(),
      code,
      title,
      detail,
      stack,
    )
  }
  pub fn internal(locale: &str, code: &str, stack: Error) -> Self {
    Self::internal_raw(
      locale,
      code,
      &I18N
        .read()
        .unwrap()
        .with_lang("internal-error-title", locale),
      Some(
        &I18N
          .read()
          .unwrap()
          .with_lang("internal-error-detail", locale),
      ),
      stack,
    )
  }
  pub fn bad_request(locale: &str, code: &str, stack: Error) -> Self {
    Self::bad_request_raw(
      locale,
      code,
      &I18N.read().unwrap().with_lang("bad-request-title", locale),
      Some(&I18N.read().unwrap().with_lang("bad-request-detail", locale)),
      stack,
    )
  }
  pub fn unauthorized(locale: &str, code: &str, stack: Error) -> Self {
    Self::unauthorized_raw(
      locale,
      code,
      &I18N.read().unwrap().with_lang("unauthorized-title", locale),
      Some(
        &I18N
          .read()
          .unwrap()
          .with_lang("unauthorized-detail", locale),
      ),
      stack,
    )
  }
  pub fn phone_code_expired(locale: &str, stack: Error) -> Self {
    let code = "phone_code_expired";
    Self::bad_request_raw(
      locale,
      code,
      &I18N.read().unwrap().with_lang(&get_title(code), locale),
      Some(&I18N.read().unwrap().with_lang(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn phone_code_failed_or_expired(locale: &str, stack: Error) -> Self {
    let code = "phone_code_expired";
    Self::bad_request_raw(
      locale,
      code,
      &I18N.read().unwrap().with_lang(&get_title(code), locale),
      Some(&I18N.read().unwrap().with_lang(&get_detail(code), locale)),
      stack,
    )
  }
  pub fn get_phone_code_too_many_requests(locale: &str, stack: Error) -> Self {
    let code = "get_phone_code_too_many_requests";
    Self::too_many_requests_raw(
      locale,
      code,
      &I18N.read().unwrap().with_lang(&get_title(code), locale),
      Some(&I18N.read().unwrap().with_lang(&get_detail(code), locale)),
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
