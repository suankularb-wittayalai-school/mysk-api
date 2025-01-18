use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationType {
    first_p: u32,
    last_p: u32,
    next_p: Option<u32>,
    prev_p: Option<u32>,
    size: u32,
    total: u32,
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

impl Display for PaginationType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ first: {}, last: {}, next: {:?}, prev: {:?}, size: {}, total: {} }}",
            self.first_p, self.last_p, self.next_p, self.prev_p, self.size, self.total,
        )
    }
}

#[derive(Debug, Serialize)]
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
            self.timestamp, self.pagination,
        )
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct EmptyResponseData;

#[derive(Debug, Serialize)]
pub struct ErrorType {
    pub id: Uuid,
    pub code: i64,
    pub error_type: String,
    pub detail: String,
    pub source: String,
}

impl ErrorType {
    // TODO
    pub async fn log(&self, pool: &PgPool, api_key: Option<Uuid>) {
        query!(
            "
            INSERT INTO api_logging.error_logs (id, code, error_type, detail, source, api_key_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
            self.id,
            self.code,
            self.error_type,
            self.detail,
            self.source,
            api_key,
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, code: {}, error_type: {}, detail: {}, source: {} }}",
            self.id, self.code, self.error_type, self.detail, self.source,
        )
    }
}

#[derive(Debug, Serialize)]
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
            self.api_version, self.error, self.data, self.meta,
        )
    }
}
