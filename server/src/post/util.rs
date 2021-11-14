use crate::{
  constant::POST_TEMPLATE_BLANK_MARK,
  error::{Error, ServiceError},
  global::{Config, SensitiveWords},
  middleware::Locale,
  post::model::Visibility,
  types::ServiceResult,
};
pub fn is_post_content_valid(locale: &Locale, content: &str) -> ServiceResult<()> {
  let min_post_content_count = Config::global().post.min_post_content_count;
  if content.chars().count() < min_post_content_count as usize {
    return Err(ServiceError::param_invalid(
      locale,
      "invalid_content_count",
      Error::Default,
    ));
  }
  return Ok(());
}
// spam post check
// like 留号
// 其他社交平台的留号
// 仅自己可见
pub fn get_post_content_visibility(content: &str, original_visibility: Visibility) -> Visibility {
  let sensitive_words = &SensitiveWords::global().0;
  let spam_result = sensitive_words.iter().find(|&word| content.contains(word));
  if let Some(keyword) = spam_result {
    tracing::info!("sensitive keyword: {}, set it as private", keyword);
    return Visibility::Private;
  } else {
    return original_visibility;
  }
}
pub fn is_post_template_content_valid(locale: &Locale, content: &str) -> ServiceResult<()> {
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
