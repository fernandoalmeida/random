use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DieselResultError};
use uuid::parser::ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("internal server error, please try later")
            },
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("unauthorized"),
        }
    }
}

impl From<DieselResultError> for ServiceError {
    fn from(error: DieselResultError) -> ServiceError {
        match error {
            DieselResultError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }

                ServiceError::InternalServerError
            },
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<ParseError> for ServiceError {
    fn from(_error: ParseError) -> ServiceError {
        ServiceError::BadRequest("invalid UUID".into())
    }
}
