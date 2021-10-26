use crate::{account::route::service_route as account_service_route, constant::API_V1_PREFIX};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexMeta {
  pub version: String,
  pub prefix: String,
}

pub fn app_route() -> Router {
  let route = Router::new()
    .route(
      "/",
      get(|| async {
        Json(IndexMeta {
          version: "v1".to_string(),
          prefix: API_V1_PREFIX.to_string(),
        })
      }),
    )
    .nest(
      API_V1_PREFIX,
      Router::new().nest("/account", account_service_route()),
    );
  route
}
