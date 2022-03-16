use crate::{
    account::route::service_route as account_service_route,
    constant::{
        ACCOUNT_SERVICE_PATH, API_V1_PREFIX, FILE_SERVICE_PATH, NOTIFICATION_SERVICE_PATH,
        POST_SERVICE_PATH, REPORT_SERVICE_PATH,
    },
    file::route::service_route as file_service_route,
    middleware::{ClientPlatform, ClientVersion, Signature},
    notification::route::{push_forward_handler, service_route as notification_service_route},
    post::{page::post::get_post_page_handler, route::service_route as post_service_route},
    report::route::service_route as report_service_route,
};

use axum::{
    extract::extractor_middleware,
    http::StatusCode,
    routing::{get, get_service, post},
    Json, Router,
};
use jsonapi::api::{DocumentData, JsonApiDocument, JsonApiInfo, JsonApiValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tower_http::services::ServeDir;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexMeta {
    pub version: String,
    pub prefix: String,
    pub protocol: String,
}

pub fn app_route() -> Router {
    let mut meta: HashMap<String, JsonApiValue> = HashMap::new();
    meta.insert("prefix".to_string(), json!(API_V1_PREFIX));
    meta.insert(
        "build_date".to_string(),
        json!(env!("VERGEN_BUILD_TIMESTAMP")),
    );
    meta.insert(
        "server_version".to_string(),
        json!(env!("VERGEN_BUILD_SEMVER")),
    );
    meta.insert("server_hash".to_string(), json!(env!("VERGEN_GIT_SHA")));
    meta.insert(
        "server_commit_date".to_string(),
        json!(env!("VERGEN_GIT_COMMIT_TIMESTAMP")),
    );
    // v3 for im push services
    let route = Router::new()
        .route(
            "/",
            get(|| async {
                let data = DocumentData {
                    jsonapi: Some(JsonApiInfo {
                        version: Some("1.0".to_string()),
                        meta: Some(meta),
                    }),
                    ..Default::default()
                };

                let doc = JsonApiDocument::Data(data);
                Json(doc)
            }),
        )
        .route(
            "/v3/notification/:registration_id",
            post(push_forward_handler),
        )
        .route("/post", get(get_post_page_handler))
        .nest(
            API_V1_PREFIX,
            Router::new()
                .nest(ACCOUNT_SERVICE_PATH, account_service_route())
                .nest(POST_SERVICE_PATH, post_service_route())
                .nest(NOTIFICATION_SERVICE_PATH, notification_service_route())
                .nest(REPORT_SERVICE_PATH, report_service_route())
                .nest(FILE_SERVICE_PATH, file_service_route())
                .route_layer(extractor_middleware::<ClientVersion>())
                .route_layer(extractor_middleware::<ClientPlatform>())
                .route_layer(extractor_middleware::<Signature>()),
        )
        .fallback(
            get_service(ServeDir::new("./resources/static")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        );

    route
}
