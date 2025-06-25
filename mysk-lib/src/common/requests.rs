use crate::{common::PaginationConfig, prelude::*, query::Queryable};
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use futures::future;
use serde::{Deserialize, de::DeserializeOwned};
use sqlx::{Postgres, QueryBuilder};
use std::fmt::{Display, Formatter};
use std::string::ToString;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FetchLevel {
    IdOnly,
    Compact,
    Default,
    Detailed,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FilterConfig<T> {
    pub data: Option<T>,
    pub q: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortingConfig<S: Display> {
    by: Vec<S>,
    ascending: Option<bool>,
}

impl<S: Display> SortingConfig<S> {
    pub fn new(by: Vec<S>, ascending: Option<bool>) -> Self {
        Self { by, ascending }
    }

    pub fn append_into_query_builder(&self, qb: &mut QueryBuilder<'_, Postgres>) {
        qb.push(" ORDER BY ")
            .push(
                self.by
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .push(if self.ascending.unwrap_or(true) {
                " ASC"
            } else {
                " DESC"
            });
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortablePlaceholder;

impl Display for SortablePlaceholder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestType<T, Q: Queryable, S: Display> {
    pub data: Option<T>,
    pub pagination: Option<PaginationConfig>,
    pub filter: Option<FilterConfig<Q>>,
    pub sort: Option<SortingConfig<S>>,
    pub fetch_level: Option<FetchLevel>,
    pub descendant_fetch_level: Option<FetchLevel>,
}

// Implement from request for `RequestType` with any `T`, `Q`, and `S`
impl<T, Q, S> FromRequest for RequestType<T, Q, S>
where
    T: DeserializeOwned,
    Q: DeserializeOwned + Queryable,
    S: DeserializeOwned + Display,
{
    type Error = Error;
    type Future = future::Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string();
        let qs_parser = serde_qs::Config::new(5, false);
        let request_query = qs_parser.deserialize_str::<RequestType<T, Q, S>>(query_string);

        match request_query {
            Ok(query) => future::ok(query),
            Err(e) => future::err(Error::InvalidRequest(e.to_string(), req.path().to_string())),
        }
    }
}
