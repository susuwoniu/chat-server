use crate::{
    account::{
        model::{IdentityType, PutDeviceParam, SignupData, SignupParam},
        service::devices::put_device,
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    global::{Config, I18n},
    im::{model::ImSignupParam, service::signup::signup as im_signup},
    middleware::Locale,
    types::ServiceResult,
    util::{id::next_id, string::get_random_letter},
};
use sonyflake::Sonyflake;

use chrono::Utc;
use fluent_bundle::FluentArgs;
use sqlx::query;
pub async fn signup(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    param: SignupParam,
    sf: &mut Sonyflake,
) -> ServiceResult<SignupData> {
    let account_id = next_id(sf);
    let cfg = Config::global();
    let now = Utc::now();
    // get random name
    let mut args = FluentArgs::new();
    args.set("random", get_random_letter(4));
    let default_name = I18n::global().with_args("default-name", &locale, args);
    let mut tx = pool.begin().await?;
    let SignupParam {
        phone_country_code,
        phone_number,
        identity_type,
        identifier,
        timezone_in_seconds,
        ip,
        client_platform,
        admin,
        device_token,
        push_service_type,
    } = param;

    if identity_type == IdentityType::Phone
        && (phone_country_code.is_none() || phone_number.is_none())
    {
        // must suppliy
        return Err(ServiceError::bad_request(
            locale,
            "phone_empty",
            Error::Default,
        ));
    }
    let mut approved_at = None;
    let approved = !cfg.invite_only;
    if approved {
        approved_at = Some(now.naive_utc());
    }
    // add acccount
    query!(
    r#"
INSERT INTO accounts (id,name,phone_country_code,phone_number,updated_at,timezone_in_seconds,approved,approved_at,admin)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
"#,
    account_id,
    default_name,
    phone_country_code,
    phone_number,
    Utc::now().naive_utc(),
    timezone_in_seconds,
    approved,
    approved_at,
    admin
  )
  .execute(&mut tx)
  .await?;
    let account_auth_id = next_id(sf);

    // add account_auths
    // TODO source_from, sign_up_ip, invite_id
    query!(
        r#"
INSERT INTO account_auths (id,identity_type,identifier,account_id,updated_at,signup_ip)
VALUES ($1,$6,$2,$3,$4,$5)
"#,
        account_auth_id,
        identifier,
        account_id,
        now.naive_utc(),
        ip,
        IdentityType::Phone as _
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    // sign up im user

    // insert device data
    if let Some(device_token) = device_token {
        if let Some(push_service_type) = push_service_type.clone() {
            put_device(
                locale,
                pool,
                kv,
                PutDeviceParam {
                    account_id: Some(account_id),
                    device_token: device_token.clone(),
                    push_service_type: push_service_type,
                    client_platform: client_platform.clone(),
                },
                sf,
            )
            .await?;
        }
    }
    im_signup(
        locale,
        ImSignupParam {
            account_id,
            try_login: true,
            client_platform: client_platform.into(),
            name: default_name,
            avatar: None,
        },
    )
    .await?;
    Ok(SignupData {
        account_id,
        account_auth_id,
    })
}
