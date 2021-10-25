use crate::account::route::service_route as account_service_route;
use axum::Router;

pub fn app_route() -> Router {
  let route = Router::new().nest(
    "/api/v1",
    Router::new().nest("/account", account_service_route()),
  );
  route
}
