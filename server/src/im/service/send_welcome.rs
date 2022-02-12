use crate::{
    global::Config, im::service::send_message::send_message, middleware::Locale,
    types::ServiceResult,
};
pub async fn welcome(locale: &Locale, target_account_id: i64) -> ServiceResult<()> {
    let cfg = Config::global();

    send_message(
        locale,
        cfg.im.team_account_id,
        target_account_id,
        cfg.im.welcome_message.to_string(),
    )
    .await?;

    Ok(())
}
