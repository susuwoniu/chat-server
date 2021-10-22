use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use config::ConfigError;
use deadpool_redis::redis::RedisError;
use deadpool_redis::CreatePoolError;
use derive_more::Display;
use sqlx::Error as SqlxError;
use std::convert::From;
use std::io;
use thiserror::Error as ThisError;
pub mod definition;
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
  #[error("{0}")]
  Other(String),
  #[error("unknown data store error")]
  Unknown,
}

#[derive(Display, Debug, Serialize, Clone)]
#[display(
  fmt = "status: {}, code: {}, title: {}, detail: {}",
  status,
  code,
  title,
  detail
)]
pub struct ServiceError {
  pub status: u16,
  pub code: String,
  pub title: String,
  pub detail: String,
}
#[derive(Display, Debug, Serialize, Clone)]
#[display(fmt = "errors: {:?}", errors)]
pub struct RootError {
  pub errors: Vec<ServiceError>,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
  fn error_response(&self) -> HttpResponse {
    return HttpResponse::build(
      StatusCode::from_u16(self.status).expect("invalid http status code"),
    )
    .json(RootError {
      errors: vec![self.clone()],
    });
  }
}

impl ResponseError for RootError {
  fn error_response(&self) -> HttpResponse {
    if self.errors.len() > 0 {
      return HttpResponse::build(
        StatusCode::from_u16(self.errors[0].status).expect("invalid http status code"),
      )
      .json(self);
    } else {
      return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(RootError {
        errors: vec![ServiceError::internal(
          "zh-Hans",
          "unknown_internal_error",
          Error::Other("errors array empty".to_string()),
        )],
      });
    }
  }
}

impl From<SqlxError> for ServiceError {
  fn from(error: SqlxError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::internal("zh-Hans", "database_error", error.into())
  }
}
impl From<serde_json::Error> for ServiceError {
  fn from(error: serde_json::Error) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request("zh-Hans", "parse_json_failed", error.into())
  }
}

impl From<chrono::ParseError> for ServiceError {
  fn from(error: chrono::ParseError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    ServiceError::bad_request("zh-Hans", "parse_time_failed", error.into())
  }
}
impl From<CreatePoolError> for ServiceError {
  fn from(error: CreatePoolError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error
    ServiceError::internal("zh-Hans", "redis_connection_error", error.into())
  }
}

impl From<deadpool::managed::PoolError<deadpool_redis::redis::RedisError>> for ServiceError {
  fn from(error: deadpool::managed::PoolError<deadpool_redis::redis::RedisError>) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    ServiceError::internal("zh-Hans", "dealpool_redis_connection_error", error.into())
  }
}
impl From<RedisError> for ServiceError {
  fn from(error: RedisError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    ServiceError::internal("zh-Hans", "redis_error", error.into())
  }
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
