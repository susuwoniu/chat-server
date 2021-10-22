use super::signin;
use super::util::get_phone_code_temp_key;
use crate::account::model::{AuthData, LoginActivityData, PhoneAuthPostData};
use crate::error::{Error, ServiceError, ServiceResult};
use crate::i18n::I18N;
use crate::middleware::req_meta::ReqMeta;
use crate::types::{KvPool, Pool};
use crate::util::id::next_id;
use crate::util::key_pair::Pair;
use crate::util::string::get_random_letter;
use chrono::{NaiveDateTime, Utc};
use deadpool_redis::redis::cmd;
use fluent_bundle::FluentArgs;
use sqlx::query;
pub async fn login_with_phone(
    req_meta: ReqMeta,
    phone_auth_post_data: PhoneAuthPostData,
    pool: &Pool,
    kv: &KvPool,
    pair: &Pair,
) -> ServiceResult<AuthData> {
    // verify code
    // get kv value

    let temp_key = get_phone_code_temp_key(
        phone_auth_post_data.phone_country_code.clone(),
        phone_auth_post_data.phone_number.clone(),
    );
    let mut conn = kv.get().await?;
    let code_option: Option<String> = cmd("GET").arg(&temp_key).query_async(&mut conn).await?;
    if let Some(code) = code_option {
        if code == phone_auth_post_data.code {
            // del the key
            cmd("DEL").arg(&temp_key).query_async(&mut conn).await?;
            // first check is registered.
            let identifier = format!(
                "{}{}",
                &phone_auth_post_data.phone_country_code, &phone_auth_post_data.phone_number
            );
            // let identify_type = IdentityType::Phone;
            let account_auth_row = query!(
                "select id, account_id,current_signin_at,disabled from account_auths where identifier = $1 and identity_type = 'phone' and deleted = false",
                identifier,
            )
            .fetch_optional(pool)
            .await?;
            // TODO check disabled

            if let Some(account_auth) = account_auth_row {
                // yes, user exists, login
                signin(
                    req_meta,
                    LoginActivityData {
                        account_id: account_auth.account_id,
                        account_auth_id: account_auth.id,
                        last_signin_at: account_auth
                            .current_signin_at
                            .unwrap_or(NaiveDateTime::from_timestamp(0, 0)),
                    },
                    pool,
                    kv,
                    pair,
                )
                .await
            } else {
                // signup and login
                let account_id = next_id();
                // get random name
                let mut args = FluentArgs::new();
                args.set("random", get_random_letter(4));
                let default_name =
                    &I18N
                        .read()
                        .unwrap()
                        .with_args("default-name", &req_meta.locale, args);
                let mut tx = pool.begin().await?;
                // add acccount
                query!(
                    r#"
INSERT INTO accounts (id,name,phone_country_code,phone_number,updated_at)
VALUES ($1,$2,$3,$4,$5)
"#,
                    account_id,
                    default_name,
                    phone_auth_post_data.phone_country_code,
                    phone_auth_post_data.phone_number,
                    Utc::now().naive_utc()
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
                    req_meta,
                    LoginActivityData {
                        account_id: account_id,
                        account_auth_id: account_auth_id,
                        last_signin_at: NaiveDateTime::from_timestamp(0, 0),
                    },
                    pool,
                    kv,
                    pair,
                )
                .await
            }
        } else {
            return Err(ServiceError::phone_code_failed_or_expired(
                &req_meta.locale,
                Error::Other(format!(
                    "Can not match code of {:?} from cache ",
                    &phone_auth_post_data
                )),
            ));
        }
    } else {
        return Err(ServiceError::phone_code_expired(
            &req_meta.locale,
            Error::Other(format!(
                "Can not get code of {:?} from cache ",
                &phone_auth_post_data
            )),
        ));
    }
}
