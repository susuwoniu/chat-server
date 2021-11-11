use crate::{
  constant::POST_TEMPLATE_BLANK_MARK,
  error::{Error, ServiceError},
  middleware::Locale,
  types::ServiceResult,
};

pub fn is_post_template_content_valid(locale: &Locale, content: String) -> ServiceResult<()> {
  if !content.contains(POST_TEMPLATE_BLANK_MARK) {
    return Err(ServiceError::param_invalid(
      locale,
      "content_must_include_blank_mark",
      Error::Default,
    ));
  } else {
    // 保证 ____ 前后有空格，如果前后有别的字符的话
    // 这段逻辑现在是有点问题的，不太会用rust的substring定位字符

    let content_len = content.chars().count();
    let mark_len = POST_TEMPLATE_BLANK_MARK.chars().count();
    if content_len > mark_len {
      let first_char: String = content.chars().take(1).collect();
      let last_char: String = content.chars().skip(content_len - 1).take(1).collect();

      if &first_char != "_" {
        if !content.contains(&format!(" {}", POST_TEMPLATE_BLANK_MARK)) {
          return Err(ServiceError::param_invalid(
            locale,
            "content_must_include_space_before_blank_mark",
            Error::Default,
          ));
        }
      }

      if &last_char != "_" {
        if !content.contains(&format!("{} ", POST_TEMPLATE_BLANK_MARK)) {
          return Err(ServiceError::param_invalid(
            locale,
            "content_must_include_space_after_blank_mark",
            Error::Default,
          ));
        }
      }
    }
  }
  return Ok(());
}
