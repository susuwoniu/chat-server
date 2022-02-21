#![forbid(unsafe_code)]
#![feature(derive_default_enum)]

mod account;
mod alias;
mod cmd;
mod constant;
mod consumer;
mod error;
mod file;
mod global;
mod im;
mod middleware;
mod notification;
mod post;
mod report;
mod route;
mod types;
mod util;

use crate::{
    cmd::{
        args::{AdminCommand, CliOpt, ClientCommand, ServerCommand},
        create_admin, create_user, init_post_templates, set_admin,
    },
    error::Error,
    global::{
        config::{CONFIG, ENV},
        i18n::I18N,
        AccessTokenPair, Client, Config, I18n, ImClient, RefreshTokenPair, SensitiveWords,
        XmppClient,
    },
    route::app_route,
    util::{id::next_id, key_pair::Pair},
};
use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    AddExtensionLayer, BoxError,
};
use consumer::consumer;
use deadpool_redis::Config as RedisConfig;
use queue_file::QueueFile;
use sonyflake::Sonyflake;
use sqlx::postgres::PgPoolOptions;
use std::{borrow::Cow, convert::Infallible, fs, path::Path, sync::Arc, time::Duration};
use structopt::StructOpt;
use tokio::sync::Mutex;
use tower::{filter::AsyncFilterLayer, util::AndThenLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
// Our shared state
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
    // init id
    // GlobalId::init(1);
    let mut sf = Sonyflake::builder()
        .machine_id(&|| Ok(1))
        .finalize()
        .unwrap();
    // sub command
    let opt = CliOpt::from_args();
    match opt {
        CliOpt::Client(client_subcommand) => match client_subcommand {
            ClientCommand::Create => {
                let client_id = next_id(&mut sf);
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
            XmppClient::init();

            match admin_command {
                AdminCommand::CreateUser => {
                    create_user().await?;
                }
                AdminCommand::Create => {
                    create_admin().await?;
                }
                AdminCommand::Set => {
                    set_admin().await?;
                }
                AdminCommand::InitTemplates(opts) => {
                    init_post_templates(opts).await?;
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
                    XmppClient::init();

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

                    // start queue
                    // ensure parent path exists
                    let signup_queue_path = Path::new(&cfg.account.signup_queue_path);
                    let parent_path = signup_queue_path.parent().unwrap();
                    fs::create_dir_all(parent_path)?;
                    let qf = QueueFile::open(cfg.account.signup_queue_path.clone())
                        .expect("cannot open queue file");
                    let qf_mutex = Mutex::new(qf);
                    let qf_arc = Arc::new(qf_mutex);
                    let qf_arc1 = qf_arc.clone();
                    let join = tokio::spawn(async move {
                        consumer(qf_arc1).await;
                    });

                    axum::Server::bind(&addr)
                        .serve(
                            app_route()
                                .layer(
                                    ServiceBuilder::new()
                                        // Handle errors from middleware
                                        //
                                        // This middleware most be added above any fallible
                                        // ones if you're using `ServiceBuilder`, due to how ordering works
                                        .layer(HandleErrorLayer::new(handle_error))
                                        .timeout(Duration::from_secs(10))
                                        // `TraceLayer` adds high level tracing and logging
                                        .layer(TraceLayer::new_for_http())
                                        // `AsyncFilterLayer` lets you asynchronously transform the request
                                        .layer(AsyncFilterLayer::new(map_request))
                                        .layer(AndThenLayer::new(map_response))
                                        .into_inner(),
                                )
                                .layer(AddExtensionLayer::new(pool))
                                .layer(AddExtensionLayer::new(redis_pool))
                                .layer(AddExtensionLayer::new(sf))
                                // .layer(AddExtensionLayer::new(txarc))
                                .layer(AddExtensionLayer::new(qf_arc))
                                .into_make_service(),
                        )
                        .await
                        .unwrap();
                    join.await.unwrap();
                }
            }
        }
    }
    Ok(())
}
async fn map_request(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    Ok(req)
}

async fn map_response(res: Response) -> Result<Response, Infallible> {
    Ok(res)
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
