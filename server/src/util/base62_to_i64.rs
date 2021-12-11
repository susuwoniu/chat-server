use crate::{
  error::{Error, ServiceError},
  middleware::Locale,
  types::ServiceResult,
};
pub fn base62_to_i64(base62: &str) -> ServiceResult<i64> {
  if base62.len() == 0 {
    return Err(ServiceError::param_invalid(
      &Locale::default(),
      "parse_base62_to_i64_failed",
      Error::Default,
    ));
  }
  let result = bs62::decode_num(base62)
    .map_err(|_| {
      ServiceError::param_invalid(
        &Locale::default(),
        "parse_base62_to_i64_failed",
        Error::Default,
      )
    })?
    .to_u64_digits()[0] as i64;
  return Ok(result);
}
