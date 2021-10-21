use actix_web::{error::ResponseError, HttpResponse};
use config::ConfigError;
use deadpool_redis::redis::RedisError;
use deadpool_redis::CreatePoolError;
use sqlx::Error as SqlxError;
use std::convert::From;
use std::io;
use thiserror::Error;
pub mod definition;
#[derive(Error, Debug)]
pub enum InternalError {
  #[error("io error")]
  Io(#[from] io::Error),
  #[error("config load error")]
  Config(#[from] ConfigError),
  #[error("db error")]
  Db(#[from] SqlxError),
  #[error("redis error")]
  Redis(#[from] CreatePoolError),
}

#[derive(Debug, Serialize)]
pub struct ErrorObject {
  pub status: u16,
  pub code: String,
  pub title: String,
  pub detail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
  pub errors: Vec<ErrorObject>,
}

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
  #[error("Internal Server Error")]
  InternalServerError,

  #[error("BadRequest: {0}")]
  BadRequest(String),

  #[error("Unauthorized: {0}")]
  Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
  fn error_response(&self) -> HttpResponse {
    match self {
      ServiceError::InternalServerError => {
        HttpResponse::InternalServerError().json("internal erro")
      }

      ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
      ServiceError::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
    }
  }
}

impl From<SqlxError> for ServiceError {
  fn from(error: SqlxError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    match error {
      SqlxError::Database(database_error) => {
        // log
        error!("database error: {:?}", database_error);
        let message = database_error.message();
        ServiceError::BadRequest(message.to_string())
      }
      _ => ServiceError::InternalServerError,
    }
  }
}
impl From<serde_json::Error> for ServiceError {
  fn from(error: serde_json::Error) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    error!("parse json error: {:?}", error);

    ServiceError::BadRequest("parse json failed".to_string())
  }
}

impl From<chrono::ParseError> for ServiceError {
  fn from(error: chrono::ParseError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    error!("parse time error: {:?}", error);

    ServiceError::BadRequest("parse time failed, please provide a rfc3382 time".to_string())
  }
}
impl From<CreatePoolError> for ServiceError {
  fn from(redis_error: CreatePoolError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    let error_meesage = format!("{:?}", redis_error);
    error!("redis create pool error {:?}", error_meesage);
    ServiceError::InternalServerError
  }
}

impl From<deadpool::managed::PoolError<deadpool_redis::redis::RedisError>> for ServiceError {
  fn from(
    redis_error: deadpool::managed::PoolError<deadpool_redis::redis::RedisError>,
  ) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    let error_meesage = format!("{:?}", redis_error);
    error!("redis pool error: {:?}", error_meesage);
    ServiceError::InternalServerError
  }
}
impl From<RedisError> for ServiceError {
  fn from(redis_error: RedisError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    // server error

    let error_meesage = format!("{:?}", redis_error);
    error!("redis error: {:?}", error_meesage);
    ServiceError::InternalServerError
  }
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
