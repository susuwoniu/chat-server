use super::ErrorObject;
use actix_web::http::StatusCode;
impl ErrorObject {
  pub fn new(status: u16, code: &str, title: &str, detail: Option<&str>) -> Self {
    let mut the_detail: Option<String> = None;
    if the_detail.is_some() {
      the_detail = Some(the_detail.unwrap().to_string());
    }
    return ErrorObject {
      status,
      code: code.to_string(),
      title: title.to_string(),
      detail: the_detail,
    };
  }
  pub fn new_with_string(status: u16, code: String, title: String, detail: Option<String>) -> Self {
    return ErrorObject {
      status,
      code: code,
      title: title,
      detail: detail,
    };
  }
  pub fn bad_request(code: &str, title: &str, detail: Option<&str>) -> Self {
    Self::new(StatusCode::BAD_REQUEST.as_u16(), code, title, detail)
  }
  pub fn internal(code: &str, title: &str, detail: Option<&str>) -> Self {
    Self::new(
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
      code,
      title,
      detail,
    )
  }
  pub fn unauthorized(code: &str, title: &str, detail: Option<&str>) -> Self {
    Self::new(StatusCode::UNAUTHORIZED.as_u16(), code, title, detail)
  }
  pub fn too_many_requests(code: &str, title: &str, detail: Option<&str>) -> Self {
    Self::new(StatusCode::TOO_MANY_REQUESTS.as_u16(), code, title, detail)
  }
}
