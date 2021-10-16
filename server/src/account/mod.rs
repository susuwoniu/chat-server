mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/accounts").service(
        web::resource("/login_with_phone").route(web::post().to(handler::login_with_phone)),
    ));
}
