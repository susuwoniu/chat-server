use crate::{
    account::{
        model::{PutDeviceParam, SignouttParam},
        service::devices::remove_devices,
        util::get_refresh_token_key,
    },
    alias::{KvPool, Pool},
    middleware::{Auth, ClientPlatform},
    types::ServiceResult,
};
use deadpool_redis::redis::cmd;
// sign in a verified account
pub async fn signout(
    pool: &Pool,
    kv: &KvPool,
    auth: &Auth,
    param: SignouttParam,
    client_platform: ClientPlatform,
) -> ServiceResult<()> {
    let Auth {
        account_id,
        device_id,
        ..
    } = auth;
    let SignouttParam {
        device_token,
        push_service_type,
        ..
    } = param;
    // add refresh token to kv
    // add to kv
    let temp_key = get_refresh_token_key(*account_id, &device_id);
    let mut conn = kv.get().await?;
    cmd("DEL")
        .arg(&[&temp_key])
        .query_async::<_, ()>(&mut conn)
        .await?;
    // if not refresh token , so write

    if let Some(device_token) = device_token {
        if let Some(push_service_type) = push_service_type.clone() {
            remove_devices(
                pool,
                PutDeviceParam {
                    device_token,
                    push_service_type,
                    client_platform: client_platform,
                    account_id: Some(*account_id),
                },
            )
            .await?;
        }
    }

    Ok(())
}
