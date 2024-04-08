use actix_web::{dev::Payload, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};

use sqlx::Encode;
use utoipa::ToSchema;

use crate::prelude::*;

#[derive(Debug, serde::Deserialize)]
pub struct QueryablePlaceholder;

#[derive(Debug, serde::Deserialize)]
pub struct SortablePlaceholder;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FetchLevel {
    IdOnly,
    Compact,
    Default,
    Detailed,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct FilterConfig<T> {
    pub data: Option<T>,
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct SortingConfig<T> {
    pub by: Vec<T>,
    pub ascending: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct PaginationConfig {
    pub p: u32,
    pub size: Option<u32>,
}

impl Default for PaginationConfig {
    fn default() -> Self {
        Self {
            p: 1,
            size: Some(50),
        }
    }
}

impl PaginationConfig {
    pub fn new(p: u32, size: Option<u32>) -> Self {
        Self { p, size }
    }

    pub fn to_limit_clause(&self) -> SqlSection {
        // LIMIT $1 OFFSET $2
        SqlSection {
            sql: vec!["LIMIT ".to_string(), " OFFSET ".to_string()],
            params: vec![
                QueryParam::Int(self.size.unwrap_or(50) as i64),
                QueryParam::Int(((self.p - 1) * self.size.unwrap_or(50)) as i64),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct RequestType<T, Queryable, Sortable> {
    pub data: Option<T>,
    pub pagination: Option<PaginationConfig>,
    pub filter: Option<FilterConfig<Queryable>>,
    pub sort: Option<SortingConfig<Sortable>>,
    pub fetch_level: Option<FetchLevel>,
    pub descendant_fetch_level: Option<FetchLevel>,
}

// Implement from request for RequestType with any T, Queryable, and Sortable
impl<T, Queryable, Sortable> FromRequest for RequestType<T, Queryable, Sortable>
where
    T: serde::de::DeserializeOwned,
    Queryable: serde::de::DeserializeOwned,
    Sortable: serde::de::DeserializeOwned,
{
    type Error = Error;
    type Future = futures::future::Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string();
        let request_query = serde_qs::from_str::<RequestType<T, Queryable, Sortable>>(query_string);

        match request_query {
            Ok(query) => futures::future::ready(Ok(query)),
            Err(e) => futures::future::ready(Err(Error::InvalidRequest(
                e.to_string(),
                req.path().to_string(),
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QueryParam {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Uuid(uuid::Uuid),
    ArrayInt(Vec<i64>),
    ArrayFloat(Vec<f64>),
    ArrayString(Vec<String>),
    ArrayBool(Vec<bool>),
    ArrayUuid(Vec<uuid::Uuid>),
}

impl Encode<'_, sqlx::Postgres> for QueryParam {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            QueryParam::String(v) => {
                <String as sqlx::Encode<sqlx::Postgres>>::encode(v.to_string(), buf)
            }
            QueryParam::Int(v) => <i64 as sqlx::Encode<sqlx::Postgres>>::encode(*v, buf),
            QueryParam::Float(v) => <f64 as sqlx::Encode<sqlx::Postgres>>::encode(*v, buf),
            QueryParam::Bool(v) => <bool as sqlx::Encode<sqlx::Postgres>>::encode(*v, buf),
            QueryParam::Uuid(v) => <uuid::Uuid as sqlx::Encode<sqlx::Postgres>>::encode(*v, buf),
            QueryParam::ArrayInt(v) => v.encode_by_ref(buf),
            QueryParam::ArrayFloat(v) => v.encode_by_ref(buf),
            QueryParam::ArrayString(v) => v.encode_by_ref(buf),
            QueryParam::ArrayBool(v) => v.encode_by_ref(buf),
            QueryParam::ArrayUuid(v) => v.encode_by_ref(buf),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SqlSection {
    pub sql: Vec<String>,
    pub params: Vec<QueryParam>,
}

impl SqlSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, sql: &str, param: QueryParam) {
        self.sql.push(sql.to_string());
        self.params.push(param);
    }
}
