use super::handler;
use actix_web::web;
pub fn route(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/accounts")
      .service(web::resource("/{account_id}").route(web::get().to(handler::get_user))),
  );
}
