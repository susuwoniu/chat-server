use actix_web::{error::ResponseError, HttpResponse};
use config::ConfigError;
use sqlx::Error as SqlxError;
use std::convert::From;
use std::io;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum InternalError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("config load error")]
    Config(#[from] ConfigError),
    #[error("db error")]
    Db(#[from] SqlxError),
}

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
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

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
