use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
};

use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, PgPool};
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use super::requests::PaginationConfig;
#[derive(Serialize, Deserialize, Debug, ToSchema, FromRow)]
pub struct ErrorType {
    pub id: Uuid,
    pub code: i64,
    pub error_type: String,
    pub detail: String,
    pub source: String,
}

impl ErrorType {
    pub fn to_status_code(&self) -> StatusCode {
        match self.code {
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            405 => StatusCode::METHOD_NOT_ALLOWED,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn log(&self, pool: &PgPool, api_key: Option<Uuid>) {
        let _ = sqlx::query!(
            r#"
            INSERT INTO api_logging.error_logs (id, code, error_type, detail, source, api_key_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            self.id,
            self.code,
            self.error_type,
            self.detail,
            self.source,
            api_key
        )
        .execute(pool)
        .await;
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, code: {}, error_type: {}, detail: {}, source: {} }}",
            self.id, self.code, self.error_type, self.detail, self.source
        )
    }
}

impl std::error::Error for ErrorType {
    fn description(&self) -> &str {
        &self.detail
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PaginationType {
    first_p: u32,
    last_p: u32,
    next_p: Option<u32>,
    prev_p: Option<u32>,
    size: u32,
    total: u32,
}

impl std::fmt::Display for PaginationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ first: {}, last: {}, next: {:?}, prev: {:?}, size: {}, total: {} }}",
            self.first_p, self.last_p, self.next_p, self.prev_p, self.size, self.total
        )
    }
}

impl PaginationType {
    pub fn new(current_p: u32, size: u32, total: u32) -> Self {
        let page_count = (total as f64 / size as f64).ceil() as u32;

        PaginationType {
            first_p: 1,
            last_p: page_count,
            next_p: if current_p < page_count {
                Some(current_p + 1)
            } else {
                None
            },
            prev_p: if current_p > 1 {
                Some(current_p - 1)
            } else {
                None
            },
            size,
            total,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MetadataType {
    timestamp: DateTime<Utc>,
    pagination: Option<PaginationType>,
}

impl MetadataType {
    pub fn new(pagination: Option<PaginationType>) -> Self {
        MetadataType {
            timestamp: Utc::now(),
            pagination,
        }
    }
}

impl Default for MetadataType {
    fn default() -> Self {
        MetadataType {
            timestamp: Utc::now(),
            pagination: None,
        }
    }
}

impl std::fmt::Display for MetadataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ timestamp: {}, pagination: {:?} }}",
            self.timestamp, self.pagination
        )
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseType<T> {
    api_version: String,
    data: Option<T>,
    error: Option<String>, // Always None
    meta: MetadataType,
}

impl<T> ResponseType<T> {
    pub fn new(data: T, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ResponseType {
            api_version: version,
            data: Some(data),
            error: None,
            meta: meta.unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ErrorResponseType {
    api_version: String,
    error: ErrorType,
    data: Option<String>, // always None
    meta: Option<MetadataType>,
}

impl ErrorResponseType {
    pub fn new(error: ErrorType, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ErrorResponseType {
            api_version: version,
            error,
            data: None,
            meta,
        }
    }
}

impl std::fmt::Display for ErrorResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ api_version: {}, error: {}, data: {:?}, meta: {:?} }}",
            self.api_version, self.error, self.data, self.meta
        )
    }
}

// implement as Actix web error response type

impl error::ResponseError for ErrorResponseType {
    fn status_code(&self) -> StatusCode {
        // convert error type to status code
        self.error.to_status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        actix_web::HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}
