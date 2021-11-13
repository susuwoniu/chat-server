use crate::{
  alias::{KvPool, Pool},
  constant::POST_SERVICE_PATH,
  middleware::{Auth, ClientVersion, Ip, Locale, Qs, RefreshTokenAuth, Signature},
  post::{
    model::{
      ApiPostFilter, CreatePostParam, CreatePostTemplateParam, PostFilter, PostTemplateFilter,
      UpdatePostParam, UpdatePostTemplateParam,
    },
    service::{
      create_post::create_post,
      create_post_template::create_post_template,
      get_post::{get_post, get_posts},
      get_post_template::{get_post_template, get_post_templates},
      update_post::update_post,
      update_post_template::update_post_template,
    },
  },
  types::{JsonApiResponse, PageInfo, QuickResponse},
  util::page::{format_page_links, format_page_meta},
};

use axum::{
  extract::{Extension, OriginalUri, Path, Query},
  http::Uri,
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
      get(get_post_template_handler)
        .patch(patch_post_template_handler)
        .delete(delete_post_template_handler),
    )
    .route("/posts", post(create_post_handler).get(get_posts_handler))
    .route(
      "/posts/:id",
      get(get_post_handler)
        .patch(patch_post_handler)
        .delete(delete_post_handler),
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
async fn get_posts_handler(
  Extension(pool): Extension<Pool>,
  locale: Locale,
  Qs(filter): Qs<ApiPostFilter>,
  Query(query): Query<HashMap<String, String>>,
  uri: Uri,
) -> JsonApiResponse {
  let posts_filter = PostFilter::try_from(filter)?;
  let data = get_posts(&locale, &pool, &posts_filter).await?;
  let json_api_data = vec_to_jsonapi_resources(data.data).0;
  let response = JsonApiDocument::Data(DocumentData {
    meta: Some(format_page_meta(data.page_info.clone())),
    data: Some(PrimaryData::Multiple(json_api_data)),
    links: Some(format_page_links(
      POST_SERVICE_PATH,
      uri.path(),
      query,
      data.page_info,
    )),
    ..Default::default()
  });
  Ok(Json(response))
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
  Json(payload): Json<UpdatePostTemplateParam>,
) -> JsonApiResponse {
  let data = update_post_template(&locale, &pool, id, payload, auth).await?;
  Ok(Json(data.to_jsonapi_document()))
}
async fn patch_post_handler(
  Extension(pool): Extension<Pool>,
  Path(id): Path<i64>,
  locale: Locale,
  auth: Auth,
  Json(payload): Json<UpdatePostParam>,
) -> JsonApiResponse {
  let data = update_post(&locale, &pool, id, payload, auth).await?;
  Ok(Json(data.to_jsonapi_document()))
}
async fn delete_post_handler(
  Extension(pool): Extension<Pool>,
  Path(id): Path<i64>,
  locale: Locale,
  auth: Auth,
) -> JsonApiResponse {
  let data = update_post(
    &locale,
    &pool,
    id,
    UpdatePostParam {
      deleted: Some(true),
      ..Default::default()
    },
    auth,
  )
  .await?;
  QuickResponse::default()
}
async fn delete_post_template_handler(
  Extension(pool): Extension<Pool>,
  Path(id): Path<i64>,
  locale: Locale,
  auth: Auth,
) -> JsonApiResponse {
  let _ = update_post_template(
    &locale,
    &pool,
    id,
    UpdatePostTemplateParam {
      deleted: Some(true),
      ..Default::default()
    },
    auth,
  )
  .await?;
  QuickResponse::default()
}
