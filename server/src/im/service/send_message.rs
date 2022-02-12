use crate::{
  global::{Config, XmppClient},
  im::model::XmppMessageParam,
  middleware::Locale,
  types::ServiceResult,
};
use serde_json::json;
pub async fn send_message(_: &Locale,from_account_id:i64, target_account_id: i64,message:String) -> ServiceResult<()> {
  let cfg = Config::global();

  XmppClient::global()
      .post(
          "/messages",
          json!(XmppMessageParam {
              caller: format!("{}@{}", from_account_id, cfg.im.domain),
              to: format!("{}@{}", target_account_id, cfg.im.domain),
              body: message,
          })
          .to_string(),
      )
      .await?;
  Ok(())
}
