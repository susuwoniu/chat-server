use crate::{im::model::ImSignupParam, middleware::Locale, types::ServiceResult};

pub async fn signup(locale: &Locale, param: ImSignupParam) -> ServiceResult<()> {
  // let cfg = Config::global();
  return Ok(());
  // return Ok(ImServerTokenData {
  //   im_access_token: "fake".to_string(),
  //   im_access_token_expires_at: (Utc::now() + chrono::Duration::days(30)).naive_utc(),
  // });

  // let res: ImServerSuccessResponse<ImServerTokenInternalData> = ImClient::global()
  //   .post(
  //     "/auth/user_register",
  //     json!(ImServerSignupParam {
  //       secret: cfg.im.api_key.clone(),
  //       platform: param.platform,
  //       uid: param.account_id,
  //       name: param.name.clone(),
  //       icon: param.avatar
  //     })
  //     .to_string(),
  //   )
  //   .await?;
  // Ok(res.data.into())
}
