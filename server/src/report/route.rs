use crate::{
    alias::{KvPool, Pool},
    constant::POST_SERVICE_PATH,
    middleware::{Auth, Locale},
    report::{
        model::{ApiReportFilter, CreateReportParam, ReportFilter, UpdateReportParam},
        service::{
            create_report::create_report,
            get_report::{get_report, get_reports},
            update_report::update_report,
        },
    },
    types::JsonApiResponse,
    util::page::{format_page_links, format_page_meta},
};

use axum::{
    extract::{Extension, Path, Query},
    http::Uri,
    routing::{get, post},
    Json, Router,
};
use jsonapi::{api::*, model::*};
use sonyflake::Sonyflake;

pub fn service_route() -> Router {
    Router::new()
        .route(
            "/reports",
            post(create_report_handler).get(get_reports_handler),
        )
        .route(
            "/reports/:id",
            get(get_report_handler).patch(patch_report_handler),
        )
}

async fn create_report_handler(
    Extension(pool): Extension<Pool>,
    Extension(kv): Extension<KvPool>,
    locale: Locale,
    Json(payload): Json<CreateReportParam>,
    auth: Auth,
    Extension(mut sf): Extension<Sonyflake>,
) -> JsonApiResponse {
    let data = create_report(&locale, &pool, &kv, payload, auth, &mut sf).await?;
    Ok(Json(data.to_jsonapi_document()))
}

async fn get_report_handler(
    Extension(pool): Extension<Pool>,
    Path(id): Path<i64>,
    locale: Locale,
) -> JsonApiResponse {
    let data = get_report(&locale, &pool, id).await?;
    Ok(Json(data.to_jsonapi_document()))
}
async fn get_reports_handler(
    Extension(pool): Extension<Pool>,
    locale: Locale,
    uri: Uri,
    Query(query): Query<HashMap<String, String>>,
    Query(filter): Query<ApiReportFilter>,
) -> JsonApiResponse {
    let final_filter = ReportFilter::try_from(filter)?;
    let data = get_reports(&locale, &pool, &final_filter).await?;
    let resources = vec_to_jsonapi_resources(data.data);
    let json_api_data = resources.0;
    let other = resources.1;
    let response = JsonApiDocument::Data(DocumentData {
        meta: Some(format_page_meta(data.page_info.clone())),
        data: Some(PrimaryData::Multiple(json_api_data)),
        links: Some(format_page_links(
            POST_SERVICE_PATH,
            uri.path(),
            query,
            data.page_info,
        )),
        included: other,
        ..Default::default()
    });
    Ok(Json(response))
}
async fn patch_report_handler(
    Extension(pool): Extension<Pool>,
    Path(id): Path<i64>,
    locale: Locale,
    auth: Auth,
    Json(payload): Json<UpdateReportParam>,
) -> JsonApiResponse {
    let data = update_report(&locale, &pool, id, payload, auth, false).await?;
    Ok(Json(data.to_jsonapi_document()))
}
