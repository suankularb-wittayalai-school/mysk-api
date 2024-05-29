use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
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
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn log(&self, pool: &PgPool, api_key: Option<Uuid>) {
        let _ = sqlx::query!(
            "
            INSERT INTO api_logging.error_logs (id, code, error_type, detail, source, api_key_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
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

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginationType {
    first_p: u32,
    last_p: u32,
    next_p: Option<u32>,
    prev_p: Option<u32>,
    size: u32,
    total: u32,
}

impl Display for PaginationType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ first: {}, last: {}, next: {:?}, prev: {:?}, size: {}, total: {} }}",
            self.first_p, self.last_p, self.next_p, self.prev_p, self.size, self.total
        )
    }
}

impl PaginationType {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn new(current_p: u32, size: u32, total: u32) -> Self {
        let page_count = (f64::from(total) / f64::from(size)).ceil() as u32;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Display for MetadataType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ timestamp: {}, pagination: {:?} }}",
            self.timestamp, self.pagination
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseType<T> {
    api_version: String,
    data: Option<T>,
    error: Option<String>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorResponseType {
    api_version: String,
    error: ErrorType,
    data: Option<String>,
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

impl Display for ErrorResponseType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ api_version: {}, error: {}, data: {:?}, meta: {:?} }}",
            self.api_version, self.error, self.data, self.meta
        )
    }
}

impl ResponseError for ErrorResponseType {
    fn status_code(&self) -> StatusCode {
        self.error.to_status_code()
    }

    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}
