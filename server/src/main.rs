#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
use dotenv;
use env_logger;
use std::collections::HashMap;
mod account;
mod cli_args;
mod config;
mod constant;
mod error;
mod i18n;
mod init;
mod middleware;
mod util;
use crate::config::Client;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use cli_args::Opt;
use sqlx::postgres::PgPoolOptions;
use util::id::next_id;
use util::key_pair::Pair;
mod types;
use crate::constant::API_V1_PREFIX;
use account::handler as account_handler;
use actix_web_httpauth::middleware::HttpAuthentication;
use deadpool_redis::Config;
use middleware::auth::validator;
#[actix_web::main]
async fn main() -> Result<(), error::Error> {
    // Gets enviroment variables from `.env.example`
    dotenv::dotenv().ok();

    // Initiates error logger
    env_logger::init();
    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        Opt::from_args()
    };
    let cfg = config::Config::get();

    // sub command

    match opt {
        Opt::Init {} => {
            info!("init start");
            init::init();
            info!("init finished");
        }
        Opt::Client {} => {
            let client_id = next_id();
            let client_secret = util::password::generate();
            println!("client_id: {}", client_id);
            println!("client_secret: {}", client_secret);
        }
        Opt::Keygen {} => {
            let pair = Pair::new();
            let secret = pair.get_secret_string();
            let public = pair.get_public_string();
            println!("secret_key: {}", secret);
            println!("public_key: {}", public);
        }
        Opt::Server {} => {
            // Database
            let pool = PgPoolOptions::new()
                .max_connections(cfg.db.max_connections)
                .connect(&cfg.db.url)
                .await?;

            // let redis_client = redis::Client::open().expect("Can't create Redis client");
            let redis_config = Config::from_url(&cfg.kv.url);
            let redis_pool = redis_config.create_pool().unwrap();
            // Server port
            let port = cfg.server.port;
            let final_workders_count = cfg.workers_count.unwrap_or(num_cpus::get());
            // token key pairs
            let secret_key = cfg.auth.secret_key.clone();
            let public_key = cfg.auth.public_key.clone();
            if secret_key.is_empty() || public_key.is_empty() {
                panic!("Missing required config for auth.secret_key or auth.public_key");
            }
            let mut client_map: HashMap<i64, Client> = HashMap::new();

            let clients = cfg.clients.clone();
            for client in clients {
                client_map.insert(client.client_id, client);
            }
            // Server
            let server = HttpServer::new(move || {
                let pairs = Pair::from_string(secret_key.clone(), public_key.clone());
                let auth = HttpAuthentication::bearer(validator);

                App::new()
                    // Database
                    .wrap(Logger::default())
                    .app_data(web::Data::new(pairs))
                    .app_data(web::Data::new(client_map.clone()))
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(redis_pool.clone()))
                    // Options
                    // Unauthorized route
                    .service(
                        web::resource(format!("{}/accounts/phone-auth", API_V1_PREFIX))
                            .route(web::post().to(account_handler::login_with_phone)),
                    )
                    .service(
                        web::resource(format!("{}/accounts/phone-code", API_V1_PREFIX))
                            .route(web::post().to(account_handler::send_phone_code)),
                    )
                    .service(
                        web::resource(format!("{}/accounts/test", API_V1_PREFIX))
                            .route(web::post().to(account_handler::test)),
                    )
                    .service(
                        web::scope(API_V1_PREFIX)
                            .wrap(auth)
                            .configure(account::route),
                    )
            })
            .workers(final_workders_count)
            .bind(("0.0.0.0", port))
            .unwrap()
            // Starts server
            .run();
            eprintln!("Listening on 0.0.0.0:{}", port);
            // Awaiting server to exit
            server.await?;
        }
    }

    Ok(())
}
