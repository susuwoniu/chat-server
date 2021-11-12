use crate::{
  alias::{KvPool, Pool},
  middleware::{Auth, ClientVersion, Ip, Locale, RefreshTokenAuth, Signature},
  post::{
    model::{
      ApiUpdatePostTemplateParam, CreatePostParam, CreatePostTemplateParam, PostTemplateFilter,
      UpdatePostTemplateParam,
    },
    service::{
      create_post::create_post,
      create_post_template::create_post_template,
      get_post::get_post,
      get_post_template::{get_post_template, get_post_templates},
      update_prost_template::update_post_template,
    },
  },
  types::{JsonApiResponse, QuickResponse, SimpleMetaResponse},
};

use axum::{
  extract::{Extension, Path, Query},
  routing::{delete, get, post},
  Json, Router,
};
use jsonapi::{api::*, model::*};

pub fn service_route() -> Router {
  Router::new()
    .route(
      "/post-templates",
      post(create_post_template_handler).get(get_post_templates_handler),
    )
    .route(
      "/post-templates/:id",
      get(get_post_template_handler).patch(patch_post_template_handler),
    )
    .route("/posts", post(create_post_handler))
    .route(
      "/posts/:id",
      get(get_post_handler).patch(patch_post_template_handler),
    )
}

async fn create_post_template_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  Json(payload): Json<CreatePostTemplateParam>,
  auth: Auth,
  Ip(ip): Ip,
) -> JsonApiResponse {
  let data = create_post_template(&locale, &pool, payload, auth, ip).await?;
  Ok(Json(data.to_jsonapi_document()))
}
async fn create_post_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  Json(payload): Json<CreatePostParam>,
  auth: Auth,
  Ip(ip): Ip,
) -> JsonApiResponse {
  let data = create_post(&locale, &pool, payload, auth, ip).await?;
  Ok(Json(data.to_jsonapi_document()))
}
async fn get_post_handler(
  Extension(pool): Extension<Pool>,
  Path(id): Path<i64>,
  locale: Locale,
) -> JsonApiResponse {
  let data = get_post(&locale, &pool, id).await?;
  Ok(Json(data.to_jsonapi_document()))
}
async fn get_post_template_handler(
  Extension(pool): Extension<Pool>,
  Path(id): Path<i64>,
  locale: Locale,
) -> JsonApiResponse {
  let data = get_post_template(&locale, &pool, id).await?;
  Ok(Json(data.to_jsonapi_document()))
}
async fn get_post_templates_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  Query(filter): Query<PostTemplateFilter>,
) -> JsonApiResponse {
  let data = get_post_templates(&locale, &pool, &filter).await?;
  Ok(Json(vec_to_jsonapi_document(data)))
}
async fn patch_post_template_handler(
  Extension(pool): Extension<Pool>,
  Path(id): Path<i64>,
  locale: Locale,
  auth: Auth,
  Json(payload): Json<ApiUpdatePostTemplateParam>,
) -> JsonApiResponse {
  let ApiUpdatePostTemplateParam {
    content,
    background_color,
    featured,
  } = payload;
  update_post_template(
    &locale,
    &pool,
    UpdatePostTemplateParam {
      id,
      content,
      background_color,
      featured,
    },
    auth,
  )
  .await?;
  QuickResponse::default()
}
