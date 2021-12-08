use crate::{alias::KvPool, global::Config, im::model::ImUpdateAccountParam, types::ServiceResult};

pub async fn update_im_account(kv: &KvPool, param: ImUpdateAccountParam) -> ServiceResult<()> {
  let cfg = Config::global();
  return Ok(());
  // let im_admin_token = get_or_create_admin_im_token(kv).await?;
  // let _: ImServerSignupResponse = ImClient::global()
  //   .post_with_token(
  //     "/user/update_user_info",
  //     &im_admin_token,
  //     json!(ImServerUpdateAccountParam {
  //       uid: param.account_id,
  //       name: param.name,
  //       icon: param.avatar,
  //       operationID: get_random_letter(32)
  //     })
  //     .to_string(),
  //   )
  //   .await?;

  // Ok(())
}
