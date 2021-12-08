use crate::{error::ServiceError, middleware::Locale};
use axum::{
  body::{Bytes, Full},
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use config::ConfigError;
use deadpool_redis::redis::RedisError;
use deadpool_redis::CreatePoolError;
use derive_more::Display;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::Error as SqlxError;
use std::convert::From;
use std::convert::Infallible;
use std::{collections::HashMap, io};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
  #[error("io error")]
  Io(#[from] io::Error),
  #[error("config load error")]
  Config(#[from] ConfigError),
  #[error("db error")]
  Db(#[from] SqlxError),
  #[error("redis error")]
  CreateRedisPool(#[from] CreatePoolError),
  #[error("parse json error")]
  ParseJson(#[from] serde_json::Error),
  #[error("parse time error")]
  ParseTime(#[from] chrono::ParseError),
  #[error("redis connected pool error")]
  RedisPoolError(#[from] deadpool::managed::PoolError<deadpool_redis::redis::RedisError>),
  #[error("redis error")]
  RedisError(#[from] RedisError),
  #[error("parse path params error")]
  ParsePathParamsError(#[from] axum::extract::rejection::PathRejection),
  #[error("parse query params error")]
  ParseQueryParamsError(#[from] axum::extract::rejection::QueryRejection),
  #[error("Infallible error")]
  InfallibleError(#[from] std::convert::Infallible),
  #[error("parse header failed")]
  TypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),
  #[error("parse ip failed")]
  IpNetworkError(#[from] ipnetwork17::IpNetworkError),
  #[error("parse semver failed")]
  SemverError(#[from] semver::Error),
  #[error("parse url query failed")]
  SerdeQsError(#[from] serde_qs::Error),
  #[error("http request failed")]
  ReqwestError(#[from] reqwest::Error),
  #[error("service error")]
  ServiceError(#[from] ServiceError),
  #[error("jwt error")]
  JWTError(#[from] jsonwebtoken::errors::Error),
  #[error("{0}")]
  Other(String),
  #[error("Default Error")]
  #[allow(dead_code)]
  Default,
}

#[derive(Display, Debug, Serialize, Clone)]
#[display(fmt = "errors: {:?}", errors)]
pub struct RootError {
  pub errors: Vec<ServiceError>,
  pub meta: Option<HashMap<String, Value>>,
}
// impl IntoResponse for HeadersError {
//   type Body = Full<Bytes>;
//   type BodyError = Infallible;

//   fn into_response(self) -> Response<Self::Body> {
//     let status = StatusCode::BAD_REQUEST;
//     let body = Json(json!(RootError {
//       errors: vec![ServiceError::base_bad_request()],
//     }));
//     return (status, body).into_response();
//   }
// }

impl IntoResponse for ServiceError {
  fn into_response(self) -> Response {
    let status = self.status;
    let body = Json(json!(RootError {
      errors: vec![self.clone()],
      meta: None
    }));
    return (status, body).into_response();
  }
}

impl IntoResponse for RootError {
  fn into_response(self) -> Response {
    if self.errors.len() > 0 {
      return (self.errors[0].status, Json(json!(self))).into_response();
    } else {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        RootError {
          meta: None,
          errors: vec![ServiceError::internal(
            &Locale::default(),
            "unknown_internal_error",
            Error::Other("errors array empty".to_string()),
          )],
        },
      )
        .into_response();
    }
  }
}

impl From<axum::extract::rejection::PathRejection> for ServiceError {
  fn from(error: axum::extract::rejection::PathRejection) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request(&Locale::default(), "parse_path_params_error", error.into())
  }
}
impl From<jsonwebtoken::errors::Error> for ServiceError {
  fn from(error: jsonwebtoken::errors::Error) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::internal(&Locale::default(), "encode_jwt_failed", error.into())
  }
}

impl From<axum::extract::rejection::QueryRejection> for ServiceError {
  fn from(error: axum::extract::rejection::QueryRejection) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request(&Locale::default(), "parse_query_error", error.into())
  }
}

impl From<std::convert::Infallible> for ServiceError {
  fn from(error: std::convert::Infallible) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request(&Locale::default(), "get_querystring_error", error.into())
  }
}

impl From<axum::extract::rejection::TypedHeaderRejection> for ServiceError {
  fn from(error: axum::extract::rejection::TypedHeaderRejection) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request(&Locale::default(), "parse_header_failed", error.into())
  }
}
impl From<SqlxError> for ServiceError {
  fn from(error: SqlxError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    if let SqlxError::RowNotFound = error {
      return ServiceError::record_not_exist(&Locale::default(), "db_row_not_found", error.into());
    }
    ServiceError::internal(&Locale::default(), "database_failed", error.into())
  }
}
impl From<serde_json::Error> for ServiceError {
  fn from(error: serde_json::Error) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request(&Locale::default(), "parse_json_failed", error.into())
  }
}
impl From<reqwest::Error> for ServiceError {
  fn from(error: reqwest::Error) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::internal(&Locale::default(), "http_request_failed", error.into())
  }
}
impl From<chrono::ParseError> for ServiceError {
  fn from(error: chrono::ParseError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request(&Locale::default(), "parse_time_failed", error.into())
  }
}
impl From<CreatePoolError> for ServiceError {
  fn from(error: CreatePoolError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error
    ServiceError::internal(&Locale::default(), "redis_connection_error", error.into())
  }
}

impl From<deadpool::managed::PoolError<deadpool_redis::redis::RedisError>> for ServiceError {
  fn from(error: deadpool::managed::PoolError<deadpool_redis::redis::RedisError>) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    ServiceError::internal(
      &Locale::default(),
      "dealpool_redis_connection_error",
      error.into(),
    )
  }
}
impl From<RedisError> for ServiceError {
  fn from(error: RedisError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    ServiceError::internal(&Locale::default(), "redis_error", error.into())
  }
}
