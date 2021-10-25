use super::handler;
use actix_web::web;
pub fn route(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/accounts")
      .service(web::resource("/me").route(web::get().to(handler::get_me)))
      .service(web::resource("/{account_id}").route(web::get().to(handler::get_slim_account))),
  );
}
