use crate::account::model::{PhoneLoginData, SlimAccount};
use crate::errors::ServiceResult;
use crate::i18n::I18n;
use crate::middleware::req_meta::ReqMeta;
use crate::types::Pool;
use crate::util::id::next_id;
use crate::util::string::get_random_letter;
use actix_web::HttpRequest;
use chrono::Utc;
use fluent::FluentArgs;

pub async fn login_with_phone(
    req: HttpRequest,
    req_meta: ReqMeta,
    account_data: PhoneLoginData,
    pool: &Pool,
    i18n: &I18n,
) -> ServiceResult<SlimAccount> {
    // verify code
    let id = next_id();
    // get random name
    let mut args = FluentArgs::new();
    args.set("random", get_random_letter(4));

    let default_name = i18n.with_args("default-name", &req_meta.locale, args);
    dbg!(&default_name);
    // todo 事务
    // add acccount
    sqlx::query!(
        r#"
        INSERT INTO accounts (id,name,phone_country_code,phone_number,updated_at)
        VALUES ($1,$2,$3,$4,$5)
        "#,
        id,
        default_name,
        account_data.phone_country_code,
        account_data.phone_number,
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;
    // todo add auths table
    Ok(SlimAccount { id })
}
