#![forbid(unsafe_code)]
mod account;
mod alias;
mod cli_args;
mod constant;
mod error;
mod global;
mod middleware;
mod route;
mod types;
mod util;
use crate::{
    cli_args::CliOpt,
    error::Error,
    global::{
        access_token_pair::ACCESS_TOKEN_PAIR,
        client::CLIENT_MAP,
        config::{Client as ClientConfig, CONFIG, ENV},
        i18n::I18N,
        refresh_token_pair::REFRESH_TOKEN_PAIR,
        AccessTokenPair, Client, Config, I18n, RefreshTokenPair,
    },
    route::app_route,
    util::{id::next_id, key_pair::Pair},
};
use axum::AddExtensionLayer;
use deadpool_redis::Config as RedisConfig;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use structopt::StructOpt;
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
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
        CliOpt::Init {} => {
            tracing::info!("init start");
            tracing::info!("init finished");
        }
        CliOpt::Client {} => {
            let client_id = next_id();
            let client_secret = util::password::generate();
            println!("client_id: {}", client_id);
            println!("client_secret: {}", client_secret);
        }
        CliOpt::Keygen {} => {
            let pair = Pair::new();
            let secret = pair.get_secret_string();
            let public = pair.get_public_string();
            println!("secret_key: {}", secret);
            println!("public_key: {}", public);
        }
        CliOpt::Server {} => {
            // init global i18n
            I18N.set(I18n::new(&cfg.i18n.fallback_language)).unwrap();
            // init refresh token pair
            REFRESH_TOKEN_PAIR
                .set(RefreshTokenPair(Pair::from_string(
                    cfg.auth.refresh_token_secret_key.clone(),
                    cfg.auth.refresh_token_public_key.clone(),
                )))
                .unwrap();
            // init access token pair
            ACCESS_TOKEN_PAIR
                .set(AccessTokenPair(Pair::from_string(
                    cfg.auth.secret_key.clone(),
                    cfg.auth.public_key.clone(),
                )))
                .unwrap();
            // init client map
            let mut client_map: HashMap<i64, ClientConfig> = HashMap::new();
            let clients = cfg.clients.clone();
            for client in clients {
                client_map.insert(client.client_id, client);
            }
            CLIENT_MAP.set(Client(client_map)).unwrap();
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
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
    Ok(())
}
