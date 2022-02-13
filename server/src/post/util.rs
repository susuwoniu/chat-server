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
    let max_post_content_count = Config::global().post.max_post_content_count;
    let current_length = content.chars().count();
    if content.chars().count() < min_post_content_count as usize {
        return Err(ServiceError::min_length_error(
            locale,
            Error::Default,
            min_post_content_count,
            current_length,
        ));
    } else if content.chars().count() > max_post_content_count as usize {
        return Err(ServiceError::max_length_error(
            locale,
            Error::Default,
            max_post_content_count,
            current_length,
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
        return Ok(());
    }
}
