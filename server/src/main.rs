#![forbid(unsafe_code)]
mod account;
mod alias;
mod cmd;
mod constant;
mod error;
mod file;
mod global;
mod im;
mod middleware;
mod notification;
mod post;
mod route;
mod types;
mod util;
use crate::{
    cmd::{
        args::{AdminCommand, CliOpt, ClientCommand, ServerCommand},
        create_admin, set_admin,
    },
    error::Error,
    global::{
        config::{CONFIG, ENV},
        i18n::I18N,
        AccessTokenPair, Client, Config, I18n, ImClient, RefreshTokenPair, SensitiveWords,
    },
    route::app_route,
    util::{id::next_id, key_pair::Pair},
};
use axum::AddExtensionLayer;
use deadpool_redis::Config as RedisConfig;
use sqlx::postgres::PgPoolOptions;
use structopt::StructOpt;
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    CONFIG.set(Config::new().unwrap()).unwrap();
    let cfg = Config::global();

    if cfg.env == ENV::Dev {
        let log_level_filter = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .finish();
        tracing::subscriber::set_global_default(log_level_filter).unwrap();
    } else {
        let log_level_filter = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .finish();
        tracing::subscriber::set_global_default(log_level_filter).unwrap();
    }
    // sub command
    let opt = CliOpt::from_args();
    match opt {
        CliOpt::Client(client_subcommand) => match client_subcommand {
            ClientCommand::Create => {
                let client_id = next_id();
                let client_secret = util::password::generate();
                println!("client_id: {}", client_id);
                println!("client_secret: {}", client_secret);
            }
        },
        CliOpt::Keygen => {
            let pair = Pair::new();
            let secret = pair.get_secret_string();
            let public = pair.get_public_string();
            println!("secret_key: {}", secret);
            println!("public_key: {}", public);
        }
        CliOpt::Admin(admin_command) => {
            // init global i18n
            I18N.set(I18n::new(&cfg.i18n.fallback_language)).unwrap();
            // init refresh token pair
            RefreshTokenPair::init(
                &cfg.auth.refresh_token_secret_key,
                &cfg.auth.refresh_token_public_key,
            );
            // init access token pair
            AccessTokenPair::init(&cfg.auth.secret_key, &cfg.auth.public_key);
            // init client map
            Client::init(&cfg.clients);
            // init im client

            ImClient::init();
            match admin_command {
                AdminCommand::Create => {
                    create_admin().await?;
                }
                AdminCommand::Set => {
                    set_admin().await?;
                }
            }
        }
        CliOpt::Server(server_command) => {
            match server_command {
                ServerCommand::Start {} => {
                    // init global i18n
                    I18N.set(I18n::new(&cfg.i18n.fallback_language)).unwrap();
                    // init refresh token pair
                    RefreshTokenPair::init(
                        &cfg.auth.refresh_token_secret_key,
                        &cfg.auth.refresh_token_public_key,
                    );
                    // init access token pair
                    AccessTokenPair::init(&cfg.auth.secret_key, &cfg.auth.public_key);
                    // init client map
                    Client::init(&cfg.clients);
                    // init sensitive words
                    SensitiveWords::init();

                    // init im client

                    ImClient::init();

                    let database_url =
                        std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
                    // Database
                    let pool = PgPoolOptions::new()
                        .max_connections(cfg.db.max_connections)
                        .connect(&database_url)
                        .await?;

                    // let redis_client = redis::Client::open().expect("Can't create Redis client");
                    let redis_config = RedisConfig::from_url(&cfg.kv.url);
                    let redis_pool = redis_config.create_pool().unwrap();
                    // todo
                    // let final_workders_count = cfg.workers_count.unwrap_or(num_cpus::get());

                    let addr = cfg.server.socket_address;
                    tracing::info!("listening on http://{}", &addr);
                    axum::Server::bind(&addr)
                        .serve(
                            app_route()
                                .layer(AddExtensionLayer::new(pool))
                                .layer(AddExtensionLayer::new(redis_pool))
                                .into_make_service(),
                        )
                        .await
                        .unwrap();
                }
            }
        }
    }
    Ok(())
}
