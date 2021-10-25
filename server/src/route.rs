use crate::{account::route::service_route as account_service_route, constant::API_V1_PREFIX};
use axum::Router;

pub fn app_route() -> Router {
  let route = Router::new().nest(
    API_V1_PREFIX,
    Router::new().nest("/account", account_service_route()),
  );
  route
}
