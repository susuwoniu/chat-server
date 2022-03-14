use crate::{
    account::{
        model::{AuthData, RefreshTokenParam, SigninParam, SigninType},
        service::signin::signin,
        util::get_refresh_token_key,
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    middleware::{ClientPlatform, Locale, RefreshTokenAuth},
    types::ServiceResult,
};
use deadpool_redis::redis::cmd;
use ipnetwork17::IpNetwork;
use sonyflake::Sonyflake;
pub async fn refresh_token_to_access_token(
    locale: &Locale,
    pool: &Pool,
    kv: &KvPool,
    auth: &RefreshTokenAuth,
    param: RefreshTokenParam,
    ip: IpNetwork,
    client_platform: ClientPlatform,
    sf: &mut Sonyflake,
) -> ServiceResult<AuthData> {
    // if redis record exist
    let RefreshTokenAuth {
        account_id,
        client_id,
        ..
    } = auth;
    let RefreshTokenParam {
        device_id,
        device_token,
        push_service_type,
    } = param;
    let mut conn = kv.get().await?;
    let temp_key = get_refresh_token_key(*account_id, &device_id);
    let token_option: Option<String> = cmd("GET").arg(&temp_key).query_async(&mut conn).await?;
    if let Some(_) = token_option {
        // yes
        // auth id not need for refresh token
        return signin(
            locale,
            pool,
            kv,
            SigninParam {
                account_id: *account_id,
                client_id: *client_id,
                account_auth_id: 0,
                device_id: device_id.clone(),
                signin_type: SigninType::RefreshToken,
                ip: ip,
                client_platform,
                push_service_type,
                device_token,
            },
            sf,
        )
        .await;
    } else {
        // fail
        return Err(ServiceError::unauthorized(
            &locale,
            "refresh_token_expired",
            Error::Default,
        ));
    }
}
