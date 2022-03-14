use super::args::{InitTemplateOpts, PostTemplateItem};
use crate::{
    account::{
        model::{IdentityType, SignupParam, UpdateAccountParam},
        service::{signup::signup, update_account::update_account},
    },
    error::Error,
    global::Config,
    middleware::{Auth, ClientPlatform, Locale},
    post::{model::CreatePostTemplateParam, service::create_post_template::create_post_template},
};
use deadpool_redis::Config as RedisConfig;
use dialoguer::Input;
use ipnetwork17::IpNetwork;
use sonyflake::Sonyflake;
use sqlx::postgres::PgPoolOptions;
pub async fn create_user() -> Result<(), Error> {
    println!("start create user account");
    let phone_number: String = Input::new()
        .with_prompt("Please input the user phone number?")
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
    let mut sf = Sonyflake::builder()
        .machine_id(&|| Ok(65535))
        .finalize()
        .unwrap();
    let redis_config = RedisConfig::from_url(&cfg.kv.url);
    let kv = redis_config.create_pool().unwrap();
    let account_auth = signup(
        &Locale::default(),
        &pool,
        &kv,
        SignupParam {
            identity_type: IdentityType::Phone,
            identifier,
            phone_country_code: Some(phone_country_code),
            phone_number: Some(phone_number),
            timezone_in_seconds: 28800,
            ip: "127.0.0.1".parse::<IpNetwork>().unwrap(),
            client_platform: ClientPlatform::IOS,
            admin: false,
            device_token: None,
            push_service_type: None,
        },
        &mut sf,
    )
    .await?;
    println!("account_auth: {:?}", account_auth);
    Ok(())
}

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
    let mut sf = Sonyflake::builder()
        .machine_id(&|| Ok(65535))
        .finalize()
        .unwrap();
    let redis_config = RedisConfig::from_url(&cfg.kv.url);
    let kv = redis_config.create_pool().unwrap();
    let account_auth = signup(
        &Locale::default(),
        &pool,
        &kv,
        SignupParam {
            identity_type: IdentityType::Phone,
            identifier,
            phone_country_code: Some(phone_country_code),
            phone_number: Some(phone_number),
            timezone_in_seconds: 28800,
            ip: "127.0.0.1".parse::<IpNetwork>().unwrap(),
            client_platform: ClientPlatform::IOS,
            admin: true,
            device_token: None,
            push_service_type: None,
        },
        &mut sf,
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

pub async fn init_post_templates(opts: InitTemplateOpts) -> Result<(), Error> {
    println!("start to init post templates");

    let cfg = Config::global();
    let account_id_value = cfg.admin.default_account_id.clone();
    if account_id_value.is_none() {
        println!("please set admin account id in config, admin -> default_account_id");
        return Ok(());
    }
    // read file
    let file_content = std::fs::read_to_string(&opts.file)?;
    let post_templates: Vec<PostTemplateItem> = serde_yaml::from_str(&file_content)?;
    let account_id = account_id_value.unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    let mut sf = Sonyflake::builder()
        .machine_id(&|| Ok(65535))
        .finalize()
        .unwrap();
    // Database
    let pool = PgPoolOptions::new()
        .max_connections(cfg.db.max_connections)
        .connect(&database_url)
        .await?;
    let redis_config = RedisConfig::from_url(&cfg.kv.url);
    let redis_pool = redis_config.create_pool().unwrap();
    // get init temlates

    for post_template in post_templates {
        let title;
        let mut content: Option<String> = None;
        match post_template {
            PostTemplateItem::TitleWithContent(the_title) => {
                // title and content
                title = the_title.clone();
                content = Some(the_title);
            }
            PostTemplateItem::OnlyTitle(the_arr) => {
                title = the_arr[0].clone();
            }
        }
        create_post_template(
            &Locale::default(),
            &pool,
            &redis_pool,
            CreatePostTemplateParam {
                title: title,
                content: content,
                featured: Some(true),
            },
            Auth {
                account_id: account_id.parse::<i64>().unwrap(),
                client_id: 377844742802649603,
                token_id: 377844742802649603,
                device_id: "admin_cmd".to_string(),
                admin: true,
                moderator: false,
                vip: false,
            },
            IpNetwork::V4(std::net::Ipv4Addr::new(127, 0, 0, 1).into()),
            &mut sf,
        )
        .await?;
    }

    Ok(())
}
