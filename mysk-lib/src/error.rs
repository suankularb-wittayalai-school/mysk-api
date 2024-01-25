use actix_web::HttpResponse;
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
}

impl From<Error> for HttpResponse {
    fn from(error: Error) -> Self {
        match error {
            Error::EntityNotFound(detail, source) => {
                HttpResponse::NotFound().json(ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 404,
                        error_type: "entity_not_found".to_string(),
                        detail,
                        source,
                    },
                    None,
                ))
            }
            Error::InternalSeverError(detail, source) => {
                HttpResponse::InternalServerError().json(ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 500,
                        error_type: "internal_server_error".to_string(),
                        detail,
                        source,
                    },
                    None,
                ))
            }
            Error::InvalidToken(detail, source) => {
                HttpResponse::Unauthorized().json(ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 401,
                        error_type: "invalid_token".to_string(),
                        detail,
                        source,
                    },
                    None,
                ))
            }
            Error::MissingToken(detail, source) => {
                HttpResponse::Unauthorized().json(ErrorResponseType::new(
                    ErrorType {
                        id: Uuid::new_v4().to_string(),
                        code: 401,
                        error_type: "missing_token".to_string(),
                        detail,
                        source,
                    },
                    None,
                ))
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Error::EntityNotFound(detail, source) => format!(
                "Entity not found: {} (source: {})",
                detail.to_string(),
                source.to_string()
            ),
            Error::InternalSeverError(detail, source) => format!(
                "Internal server error: {} (source: {})",
                detail.to_string(),
                source.to_string()
            ),
            Error::InvalidToken(detail, source) => format!(
                "Invalid token: {} (source: {})",
                detail.to_string(),
                source.to_string()
            ),
            Error::MissingToken(detail, source) => format!(
                "Missing token: {} (source: {})",
                detail.to_string(),
                source.to_string()
            ),
        };
        write!(f, "{}", error)
    }
}
