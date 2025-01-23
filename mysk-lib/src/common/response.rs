use crate::common::PaginationType;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct MetadataType {
    timestamp: DateTime<Utc>,
    pagination: Option<PaginationType>,
}

impl Default for MetadataType {
    fn default() -> Self {
        MetadataType {
            timestamp: Utc::now(),
            pagination: None,
        }
    }
}

impl MetadataType {
    pub fn new(pagination: Option<PaginationType>) -> Self {
        MetadataType {
            timestamp: Utc::now(),
            pagination,
        }
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
    // TODO: tracing and error logging
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
