use crate::models::traits::Queryable;
use crate::{models::enums::SubmissionStatus, prelude::*};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{Encode, Postgres};
use std::fmt::{Display, Formatter};
use std::string::ToString;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct QueryablePlaceholder;

impl Queryable for QueryablePlaceholder {
    fn to_query_string(&self) -> Vec<SqlSection> {
        unimplemented!("QueryablePlaceholder can't actually be queried");
    }
}

#[derive(Debug, Deserialize)]
pub struct SortablePlaceholder;

impl Display for SortablePlaceholder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FetchLevel {
    IdOnly,
    Compact,
    Default,
    Detailed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilterConfig<T> {
    pub data: Option<T>,
    pub q: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SortingConfig<T> {
    pub by: Vec<T>,
    pub ascending: Option<bool>,
}

impl<SortingObject> SortingConfig<SortingObject>
where
    SortingObject: Display,
{
    pub fn new(by: Vec<SortingObject>, ascending: Option<bool>) -> Self {
        Self { by, ascending }
    }

    pub fn to_order_by_clause(&self) -> String {
        let mut order_by = " ORDER BY ".to_string();
        let columns = self
            .by
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ");
        order_by.push_str(&columns);

        if let Some(ascending) = self.ascending {
            order_by.push_str(if ascending { " ASC" } else { " DESC" });
        }

        order_by
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

    pub fn to_limit_clause(&self) -> Result<SqlSection> {
        if self.p == 0 {
            return Err(Error::InvalidRequest(
                "Page number must be greater than zero".to_string(),
                "PaginationConfig::to_limit_clause".to_string(),
            ));
        }

        // LIMIT $1 OFFSET $2
        Ok(SqlSection {
            sql: vec!["LIMIT ".to_string(), " OFFSET ".to_string()],
            params: vec![
                QueryParam::Int(i64::from(self.size.unwrap_or(50))),
                QueryParam::Int(i64::from((self.p - 1) * self.size.unwrap_or(50))),
            ],
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestType<T, Queryable, Sortable>
where
    Sortable: Display,
{
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
    T: DeserializeOwned,
    Queryable: DeserializeOwned,
    Sortable: DeserializeOwned + Display,
{
    type Error = Error;
    type Future = future::Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string();
        let qs_parser = serde_qs::Config::new(5, false);
        let request_query =
            qs_parser.deserialize_str::<RequestType<T, Queryable, Sortable>>(query_string);

        match request_query {
            Ok(query) => future::ok(query),
            Err(e) => future::err(Error::InvalidRequest(e.to_string(), req.path().to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum QueryParam {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Uuid(Uuid),
    ArrayInt(Vec<i64>),
    ArrayFloat(Vec<f64>),
    ArrayString(Vec<String>),
    ArrayBool(Vec<bool>),
    ArrayUuid(Vec<Uuid>),
    SubmissionStatus(SubmissionStatus),
}

impl Encode<'_, Postgres> for QueryParam {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            QueryParam::String(v) => <String as sqlx::Encode<Postgres>>::encode(v.to_string(), buf),
            QueryParam::Int(v) => <i64 as sqlx::Encode<Postgres>>::encode(*v, buf),
            QueryParam::Float(v) => <f64 as sqlx::Encode<Postgres>>::encode(*v, buf),
            QueryParam::Bool(v) => <bool as sqlx::Encode<Postgres>>::encode(*v, buf),
            QueryParam::Uuid(v) => <Uuid as sqlx::Encode<Postgres>>::encode(*v, buf),
            QueryParam::ArrayInt(v) => v.encode_by_ref(buf),
            QueryParam::ArrayFloat(v) => v.encode_by_ref(buf),
            QueryParam::ArrayString(v) => v.encode_by_ref(buf),
            QueryParam::ArrayBool(v) => v.encode_by_ref(buf),
            QueryParam::ArrayUuid(v) => v.encode_by_ref(buf),
            QueryParam::SubmissionStatus(v) => {
                <SubmissionStatus as sqlx::Encode<Postgres>>::encode(*v, buf)
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
