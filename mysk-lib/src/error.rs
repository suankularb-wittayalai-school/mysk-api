use crate::common::response::{ErrorResponseType, ErrorType};
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::Error as SqlxError;
use std::{
    fmt::{Display, Formatter},
    panic,
};
use tokio::task::JoinError;
use uuid::Uuid;

#[allow(clippy::doc_markdown)]
/// Error enums for MySK API responses.
///
/// The first string is the error detail and the second string is the source.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    // Client Errors
    /// HTTP 400 - [Bad Request](https://developer.mozilla.org/docs/Web/HTTP/Status/400)
    InvalidRequest(String, String),

    /// HTTP 404 - [Not Found](https://developer.mozilla.org/docs/Web/HTTP/Status/404)
    EntityNotFound(String, String),

    /// HTTP 409 - [Conflict](https://developer.mozilla.org/docs/Web/HTTP/Status/409)
    Conflicted(String, String),

    // Authentication Errors
    /// HTTP 401 - [Unauthorized](https://developer.mozilla.org/docs/Web/HTTP/Status/401)
    MissingApiKey(String, String),

    /// HTTP 401 - [Unauthorized](https://developer.mozilla.org/docs/Web/HTTP/Status/401)
    InvalidApiKey(String, String),

    /// HTTP 401 - [Unauthorized](https://developer.mozilla.org/docs/Web/HTTP/Status/401)
    InvalidAuthorizationScheme(String, String),

    /// HTTP 401 - [Unauthorized](https://developer.mozilla.org/docs/Web/HTTP/Status/401)
    MissingToken(String, String),

    /// HTTP 401 - [Unauthorized](https://developer.mozilla.org/docs/Web/HTTP/Status/401)
    InvalidToken(String, String),

    // Authorization Error
    /// HTTP 403 - [Forbidden](https://developer.mozilla.org/docs/Web/HTTP/Status/403)
    InvalidPermission(String, String),

    // Server Errors
    /// HTTP 500 - [Internal Server Error](https://developer.mozilla.org/docs/Web/HTTP/Status/500)
    InternalServerError(String, String),

    /// HTTP 503 -
    /// [Service Unavailable](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/503)
    ServiceUnavailable(String, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            // Client Errors
            Error::InvalidRequest(detail, source) => {
                format!("Invalid request: {detail} (source: {source})")
            }
            Error::EntityNotFound(detail, source) => {
                format!("Entity not found: {detail} (source: {source})")
            }
            Error::Conflicted(detail, source) => {
                format!("Conflicted: {detail} (source: {source})")
            }
            // Authentication Errors
            Error::MissingApiKey(detail, source) => {
                format!("Missing API key: {detail} (source: {source})")
            }
            Error::InvalidApiKey(detail, source) => {
                format!("Invalid API key: {detail} (source: {source})")
            }
            Error::InvalidAuthorizationScheme(detail, source) => {
                format!("Invalid authorization scheme: {detail} (source: {source})")
            }
            Error::MissingToken(detail, source) => {
                format!("Missing token: {detail} (source: {source})")
            }
            Error::InvalidToken(detail, source) => {
                format!("Invalid token: {detail} (source: {source})")
            }
            // Authorization Error
            Error::InvalidPermission(detail, source) => {
                format!("Invalid permission: {detail} (source: {source})")
            }
            // Server Errors
            Error::InternalServerError(detail, source) => {
                format!("Internal server error: {detail} (source: {source})")
            }
            Error::ServiceUnavailable(detail, source) => {
                format!("Service unavailable: {detail} (source: {source})")
            }
        };

        write!(f, "{error}")
    }
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        if value.is_panic() {
            panic::resume_unwind(value.into_panic());
        }

        Error::InternalServerError("Internal server error".to_string(), "Tokio".to_string())
    }
}

impl From<SqlxError> for Error {
    fn from(value: SqlxError) -> Self {
        match value {
            SqlxError::Database(source) => {
                Error::InvalidRequest(source.message().to_string(), "SQLx".to_string())
            }
            SqlxError::RowNotFound => {
                Error::EntityNotFound("Entity not found".to_string(), "SQLx".to_string())
            }
            SqlxError::Io(_)
            | SqlxError::Tls(_)
            | SqlxError::PoolTimedOut
            | SqlxError::PoolClosed => {
                Error::ServiceUnavailable("Service unavailable".to_string(), "SQLx".to_string())
            }
            #[cfg(debug_assertions)]
            _ => Error::InternalServerError(value.to_string(), "SQLx".to_string()),
            #[cfg(not(debug_assertions))]
            _ => {
                Error::InternalServerError("Internal server error".to_string(), "SQLx".to_string())
            }
        }
    }
}

impl From<serde_qs::Error> for Error {
    fn from(value: serde_qs::Error) -> Self {
        #[cfg(debug_assertions)]
        return Error::InternalServerError(
            value.to_string(),
            "/auth/oauth/google (serde_qs)".to_string(),
        );

        #[cfg(not(debug_assertions))]
        return Error::InternalServerError(
            "Internal server error".to_string(),
            "/auth/oauth/google".to_string(),
        );
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        #[cfg(debug_assertions)]
        return Error::InternalServerError(value.to_string(), "reqwest".to_string());

        #[cfg(not(debug_assertions))]
        return Error::InternalServerError(
            "Internal server error".to_string(),
            "reqwest".to_string(),
        );
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        #[cfg(debug_assertions)]
        return Error::InternalServerError(value.to_string(), "jsonwebtoken".to_string());

        #[cfg(not(debug_assertions))]
        return Error::InternalServerError(
            "Internal server error".to_string(),
            "jsonwebtoken".to_string(),
        );
    }
}

impl From<&Error> for HttpResponse {
    fn from(value: &Error) -> Self {
        let response = ErrorResponseType::new(value.into(), None);

        match value {
            // Client Errors
            Error::InvalidRequest(_, _) => HttpResponse::BadRequest().json(response),
            Error::EntityNotFound(_, _) => HttpResponse::NotFound().json(response),
            Error::Conflicted(_, _) => HttpResponse::Conflict().json(response),
            // Authentication Errors
            Error::MissingApiKey(_, _)
            | Error::InvalidApiKey(_, _)
            | Error::InvalidAuthorizationScheme(_, _)
            | Error::MissingToken(_, _)
            | Error::InvalidToken(_, _) => HttpResponse::Unauthorized().json(response),
            // Authorization Error
            Error::InvalidPermission(_, _) => HttpResponse::Forbidden().json(response),
            // Server Errors
            Error::InternalServerError(_, _) => HttpResponse::InternalServerError().json(response),
            Error::ServiceUnavailable(_, _) => HttpResponse::ServiceUnavailable().json(response),
        }
    }
}

impl From<&Error> for ErrorType {
    fn from(value: &Error) -> Self {
        match value {
            // Client Errors
            Error::InvalidRequest(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 400,
                error_type: "invalid_request".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::EntityNotFound(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 404,
                error_type: "entity_not_found".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::Conflicted(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 409,
                error_type: "conflicted".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            // Authentication Errors
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
            Error::InvalidAuthorizationScheme(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 401,
                error_type: "invalid_authorization_scheme".to_string(),
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
            Error::InvalidToken(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 401,
                error_type: "invalid_token".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            // Authorization Error
            Error::InvalidPermission(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 403,
                error_type: "invalid_permission".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            // Server Errors
            Error::InternalServerError(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 500,
                error_type: "internal_server_error".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
            Error::ServiceUnavailable(detail, source) => ErrorType {
                id: Uuid::new_v4(),
                code: 503,
                error_type: "service_unavailable".to_string(),
                detail: detail.to_string(),
                source: source.to_string(),
            },
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        self.into()
    }
}
