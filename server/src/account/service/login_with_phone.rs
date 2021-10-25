use crate::{
    account::{
        model::{SigninParam, SigninWithPhoneParam},
        service::signin::signin,
        util::{get_phone_code_temp_key, AuthData},
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError, ServiceResult},
    global::I18n,
    middleware::Locale,
    util::{id::next_id, string::get_random_letter},
};

use chrono::Utc;
use deadpool_redis::redis::cmd;
use fluent_bundle::FluentArgs;
use sqlx::query;
pub async fn login_with_phone(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: &SigninWithPhoneParam,
) -> ServiceResult<AuthData> {
    // verify code
    // get kv value
    let SigninWithPhoneParam {
        phone_country_code,
        phone_number,
        code: verify_code,
        timezone_in_seconds,
        client_id,
    } = param;

    let temp_key = get_phone_code_temp_key(&phone_country_code, phone_number);
    let mut conn = kv.get().await?;
    let code_option: Option<String> = cmd("GET").arg(&temp_key).query_async(&mut conn).await?;
    if let Some(code) = code_option {
        if &code == verify_code {
            // del the key
            cmd("DEL").arg(&temp_key).query_async(&mut conn).await?;
            // first check is registered.
            let identifier = format!("{}{}", &phone_country_code, phone_number);
            let account_auth_row = query!(
                r#"select id, account_id,current_signin_at from account_auths where identifier = $1 and identity_type = 'phone' and deleted = false"#,
                identifier,
            )
            .fetch_optional(pool)
            .await?;
            // TODO check disabled
            if let Some(account_auth) = account_auth_row {
                signin(
                    locale,
                    pool,
                    kv,
                    &SigninParam {
                        account_id: account_auth.account_id,
                        account_auth_id: account_auth.id,
                        client_id: *client_id,
                    },
                )
                .await
            } else {
                // signup and login
                let account_id = next_id();
                // get random name
                let mut args = FluentArgs::new();
                args.set("random", get_random_letter(4));
                let default_name = I18n::global().with_args("default-name", &locale, args);
                let mut tx = pool.begin().await?;
                // add acccount
                query!(
                    r#"
INSERT INTO accounts (id,name,phone_country_code,phone_number,updated_at,timezone_in_seconds)
VALUES ($1,$2,$3,$4,$5,$6)
"#,
                    account_id,
                    default_name,
                    phone_country_code,
                    phone_number,
                    Utc::now().naive_utc(),
                    timezone_in_seconds
                )
                .execute(&mut tx)
                .await?;
                let account_auth_id = next_id();

                // add account_auths
                // TODO source_from, sign_up_ip, invite_id
                query!(
                    r#"
INSERT INTO account_auths (id,identity_type,identifier,account_id,updated_at)
VALUES ($1,'phone',$2,$3,$4)
"#,
                    account_auth_id,
                    identifier,
                    account_id,
                    Utc::now().naive_utc()
                )
                .execute(&mut tx)
                .await?;
                tx.commit().await?;
                // todo add auths table
                signin(
                    locale,
                    pool,
                    kv,
                    &SigninParam {
                        account_id: account_id,
                        account_auth_id: account_auth_id,
                        client_id: *client_id,
                    },
                )
                .await
            }
        } else {
            return Err(ServiceError::phone_code_failed_or_expired(
                locale,
                Error::Other(format!("Can not match code of {:?} from cache ", &param)),
            ));
        }
    } else {
        return Err(ServiceError::phone_code_expired(
            locale,
            Error::Other(format!("Can not get code of {:?} from cache ", &param)),
        ));
    }
}
