use crate::{
    account::{
        model::{
            AuthData, DeviceData, IdentityType, PutDeviceParam, SigninParam, SigninType,
            SignupData, SignupParam,
        },
        service::{signin::signin, signup::signup},
        util::get_phone_code_temp_key,
    },
    alias::{KvPool, Pool},
    error::{Error, ServiceError},
    middleware::{ClientPlatform, Locale},
    types::{PushServiceType, ServiceResult},
    util::id::next_id,
};
use chrono::Utc;
use sonyflake::Sonyflake;
use sqlx::{query, query_as};

pub async fn remove_devices(pool: &Pool, param: PutDeviceParam) -> ServiceResult<()> {
    let PutDeviceParam {
        device_token,
        push_service_type,
        client_platform,
        ..
    } = param;
    let now = Utc::now();

    query!(
        r#"
  UPDATE devices SET 
  updated_at=$1,
  deleted = true,
  deleted_at = $1
  WHERE device_token=$2 and
  service_type=$3 and
  client_platform=$4
"#,
        now.naive_utc(),
        device_token,
        push_service_type as _,
        client_platform as _,
    )
    .execute(pool)
    .await?;
    return Ok(());
}

pub async fn get_devices_by_account_id(
    _: &Locale,
    pool: &Pool,
    _: &KvPool,
    account_id: i64,
) -> ServiceResult<Vec<DeviceData>> {
    let devices = query_as!(
        DeviceData,
        r#"select id, account_id, device_token,service_type as "push_service_type:PushServiceType",client_platform as "client_platform:ClientPlatform",created_at, updated_at from devices where account_id = $1 and deleted=false and updated_at >  NOW() - INTERVAL '30 days' "#,
        account_id
    )
    .fetch_all(pool)
    .await?;
    return Ok(devices);
}

pub async fn put_device(
    _: &Locale,
    pool: &Pool,
    _: &KvPool,
    param: PutDeviceParam,
    sf: &mut Sonyflake,
) -> ServiceResult<DeviceData> {
    // verify code
    let id = next_id(sf);
    let now = Utc::now();
    // get kv value
    let PutDeviceParam {
        device_token,
        push_service_type,
        client_platform,
        account_id,
    } = param;

    //
    let device =   query_as!(DeviceData,
        r#"
  INSERT into devices 
  (id, device_token,service_type,updated_at,client_platform,account_id)
  VALUES ($1,$2,$3,$4,$5,$6) 
  ON CONFLICT (client_platform, service_type,device_token,COALESCE(deleted_at, '0001-01-01T00:00:00Z'))  DO UPDATE SET 
  updated_at=$4,
  device_token=$2,
  service_type=$3,
  client_platform=$5,
  account_id=$6
  RETURNING id,account_id,device_token,service_type as "push_service_type:PushServiceType",created_at,updated_at,client_platform as "client_platform:ClientPlatform"
"#,
        id,
        device_token,
        push_service_type as _,
        now.naive_utc(),
        client_platform as _,
        account_id
    )
    .fetch_one(pool)
    .await?;
    return Ok(device);
}
