use crate::{
    account::{
        model::{IdentityType, SignupParam, UpdateAccountParam},
        service::{signup::signup, update_account::update_account},
    },
    error::Error,
    global::Config,
    middleware::{Auth, ClientPlatform, Locale},
};
use deadpool_redis::Config as RedisConfig;
use dialoguer::Input;
use ipnetwork17::IpNetwork;
use sqlx::postgres::PgPoolOptions;

pub async fn create_admin() -> Result<(), Error> {
    println!("start create admin account");
    let phone_number: String = Input::new()
        .with_prompt("Please input the admin phone number?")
        .interact_text()?;
    let cfg = Config::global();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    let phone_country_code = 86;

    let identifier = format!("{}{}", &phone_country_code, phone_number);
    // Database
    let pool = PgPoolOptions::new()
        .max_connections(cfg.db.max_connections)
        .connect(&database_url)
        .await?;
    let account_auth = signup(
        &Locale::default(),
        &pool,
        SignupParam {
            identity_type: IdentityType::Phone,
            identifier,
            phone_country_code: Some(phone_country_code),
            phone_number: Some(phone_number),
            timezone_in_seconds: 28800,
            ip: "127.0.0.1".parse::<IpNetwork>().unwrap(),
            platform: ClientPlatform::IOS,
            admin: true,
        },
    )
    .await?;
    println!("account_auth: {:?}", account_auth);
    Ok(())
}
pub async fn set_admin() -> Result<(), Error> {
    println!("start set admin account");
    let account_id: String = Input::new()
        .with_prompt("Please input the admin account id?")
        .interact_text()?;
    let cfg = Config::global();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    // let redis_client = redis::Client::open().expect("Can't create Redis client");
    let redis_config = RedisConfig::from_url(&cfg.kv.url);
    let redis_pool = redis_config.create_pool().unwrap();
    // Database
    let pool = PgPoolOptions::new()
        .max_connections(cfg.db.max_connections)
        .connect(&database_url)
        .await?;
    let data = update_account(
        &Locale::default(),
        &pool,
        &redis_pool,
        UpdateAccountParam {
            account_id: Some(account_id.parse::<i64>().unwrap()),
            admin: Some(true),
            ..Default::default()
        },
        &Auth {
            account_id: account_id.parse::<i64>().unwrap(),
            client_id: 377844742802649603,
            token_id: 377844742802649603,
            device_id: "admin_cmd".to_string(),
            admin: true,
            moderator: false,
            vip: false,
        },
        false,
    )
    .await?;
    println!("account: {:?}", data);
    Ok(())
}
