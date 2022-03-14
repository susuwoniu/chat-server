use crate::{
    account::{
        model::{
            AuthData, IdentityType, SigninParam, SigninType, SigninWithPhoneParam, SignupData,
            SignupParam,
        },
        service::{signin::signin, signup::signup},
        util::get_phone_code_temp_key,
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    middleware::Locale,
    types::ServiceResult,
};
use deadpool_redis::redis::cmd;
use sonyflake::Sonyflake;
use sqlx::query;
pub async fn login_with_phone(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: &SigninWithPhoneParam,
    sf: &mut Sonyflake,
) -> ServiceResult<AuthData> {
    // verify code
    // get kv value
    let SigninWithPhoneParam {
        phone_country_code,
        phone_number,
        code: verify_code,
        timezone_in_seconds,
        client_id,
        device_id,
        ip,
        client_platform,
        qf_mutex,
        device_token,
        push_service_type,
    } = param;

    let temp_key = get_phone_code_temp_key(&phone_country_code, phone_number, device_id);
    let mut conn = kv.get().await?;
    let code_option: Option<String> = cmd("GET").arg(&temp_key).query_async(&mut conn).await?;
    if let Some(code) = code_option {
        if &code == verify_code {
            // del the key
            cmd("DEL").arg(&temp_key).query_async(&mut conn).await?;
            // first check is registered.
            let identifier = format!("{}{}", &phone_country_code, phone_number);
            let account_auth_row = query!(
                r#"select id, account_id,current_signin_at from account_auths where identifier = $1 and identity_type = $2 and deleted = false"#,
                identifier,
                IdentityType::Phone as _
            )
            .fetch_optional(pool)
            .await?;
            if let Some(account_auth) = account_auth_row {
                signin(
                    locale,
                    pool,
                    kv,
                    SigninParam {
                        signin_type: SigninType::PhoneCode,
                        account_id: account_auth.account_id,
                        account_auth_id: account_auth.id,
                        client_id: *client_id,
                        device_id: device_id.clone(),
                        ip: ip.clone(),
                        client_platform: client_platform.clone(),
                        device_token: device_token.clone(),
                        push_service_type: push_service_type.clone(),
                    },
                    sf,
                )
                .await
            } else {
                // signup and login
                let account_data = signup(
                    locale,
                    pool,
                    kv,
                    SignupParam {
                        identity_type: IdentityType::Phone,
                        identifier,
                        phone_country_code: Some(*phone_country_code),
                        phone_number: Some(phone_number.clone()),
                        timezone_in_seconds: *timezone_in_seconds,
                        ip: ip.clone(),
                        client_platform: client_platform.clone(),
                        admin: false,
                        device_token: device_token.clone(),
                        push_service_type: push_service_type.clone(),
                    },
                    sf,
                )
                .await?;
                let SignupData {
                    account_id,
                    account_auth_id,
                } = account_data;
                // add register queue
                qf_mutex
                    .lock()
                    .await
                    .add(format!("signup::{}::{}", locale.0, account_id).as_bytes())
                    .expect("add signup queue fail");
                // todo add auths table
                signin(
                    locale,
                    pool,
                    kv,
                    SigninParam {
                        signin_type: SigninType::SignupWithPhoneCode,
                        account_id: account_id,
                        account_auth_id: account_auth_id,
                        client_id: *client_id,
                        device_id: device_id.clone(),
                        ip: ip.clone(),
                        client_platform: client_platform.clone(),
                        device_token: device_token.clone(),
                        push_service_type: push_service_type.clone(),
                    },
                    sf,
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
