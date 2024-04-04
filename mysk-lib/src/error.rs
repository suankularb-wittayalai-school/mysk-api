use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::Display;
use uuid::Uuid;

use crate::models::common::response::{ErrorResponseType, ErrorType};

// The first string is the detail, the second string is the source
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    EntityNotFound(String, String),
    InternalSeverError(String, String),
    // Auth errors
    InvalidToken(String, String),
    MissingToken(String, String),
    MissingApiKey(String, String),
    InvalidApiKey(String, String),
}

impl From<&Error> for ErrorType {
    fn from(val: &Error) -> Self {
        match val {
            Error::EntityNotFound(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 404,
                error_type: "entity_not_found".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::InternalSeverError(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 500,
                error_type: "internal_server_error".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::InvalidToken(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 401,
                error_type: "invalid_token".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::MissingToken(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 401,
                error_type: "missing_token".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::MissingApiKey(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 401,
                error_type: "missing_api_key".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::InvalidApiKey(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 401,
                error_type: "invalid_api_key".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Error::InternalSeverError(val.to_string(), "sqlx".to_string())
    }
}

impl From<&Error> for HttpResponse {
    fn from(val: &Error) -> Self {
        let res_val: ErrorResponseType = val.into();
        match val {
            Error::EntityNotFound(_, _) => HttpResponse::NotFound().json(res_val),
            Error::InternalSeverError(_, _) => HttpResponse::InternalServerError().json(res_val),
            Error::InvalidToken(_, _) => HttpResponse::Unauthorized().json(res_val),
            Error::MissingToken(_, _) => HttpResponse::Unauthorized().json(res_val),
            Error::MissingApiKey(_, _) => HttpResponse::Unauthorized().json(res_val),
            Error::InvalidApiKey(_, _) => HttpResponse::Unauthorized().json(res_val),
        }
    }
}

impl AsRef<Error> for Error {
    fn as_ref(&self) -> &Error {
        self
    }
}

impl From<Error> for HttpResponse {
    fn from(val: Error) -> Self {
        (&val).into()
    }
}

impl From<Error> for ErrorType {
    fn from(val: Error) -> Self {
        (&val).into()
    }
}

impl From<&Error> for ErrorResponseType {
    fn from(val: &Error) -> Self {
        ErrorResponseType::new(val.into(), None)
    }
}

impl From<Error> for ErrorResponseType {
    fn from(val: Error) -> Self {
        (&val).into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Error::EntityNotFound(detail, source) => {
                format!("Entity not found: {} (source: {})", detail, source)
            }
            Error::InternalSeverError(detail, source) => {
                format!("Internal server error: {} (source: {})", detail, source)
            }
            Error::InvalidToken(detail, source) => {
                format!("Invalid token: {} (source: {})", detail, source)
            }
            Error::MissingToken(detail, source) => {
                format!("Missing token: {} (source: {})", detail, source)
            }
            Error::MissingApiKey(detail, source) => {
                format!("Missing API key: {} (source: {})", detail, source)
            }
            Error::InvalidApiKey(detailed, source) => {
                format!("Invalid API key: {} (source: {})", detailed, source)
            }
        };
        write!(f, "{}", error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        self.into()
    }
}
