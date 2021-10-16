#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
use dotenv;
use env_logger;
mod account;
mod config;
mod errors;
mod i18n;
mod middleware;
mod util;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
mod types;
#[actix_web::main]
async fn main() -> Result<(), errors::InternalError> {
    // Gets enviroment variables from `.env.example`
    dotenv::dotenv().ok();

    // Initiates error logger
    env_logger::init();

    let cfg = config::Config::new()?;
    // Database
    let pool = PgPoolOptions::new()
        .max_connections(cfg.db.max_connections)
        .connect(&cfg.db.url)
        .await?;
    // Server port
    let port = cfg.server.port;
    let final_workders_count = cfg.workers_count.unwrap_or(num_cpus::get());
    // Server
    let server = HttpServer::new(move || {
        let i18n_instance = i18n::I18n::new(&cfg.i18n.fallback_language.clone());
        App::new()
            // Database
            .app_data(web::Data::new(pool.clone()))
            // Options
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(i18n_instance))
            // Error logging
            .wrap(Logger::default())
            .wrap(middleware::req_meta::ReqMeta::new())
            .service(web::scope("/1").configure(account::route))
    })
    .workers(final_workders_count)
    .bind(("0.0.0.0", port))
    .unwrap()
    // Starts server
    .run();
    eprintln!("Listening on 0.0.0.0:{}", port);
    // Awaiting server to exit
    server.await?;
    Ok(())
}
