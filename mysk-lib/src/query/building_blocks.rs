use crate::{
    models::enums::{ContactType, ShirtSize, SubmissionStatus},
    query::SqlWhereClause,
};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

// Additional bounds to enforce strong typing.
pub trait QueryParamType {}
impl QueryParamType for i64 {}
impl QueryParamType for f64 {}
impl QueryParamType for String {}
impl QueryParamType for bool {}
impl QueryParamType for Uuid {}
impl QueryParamType for NaiveDate {}
impl QueryParamType for Vec<i64> {}
impl QueryParamType for Vec<f64> {}
impl QueryParamType for Vec<String> {}
impl QueryParamType for Vec<bool> {}
impl QueryParamType for Vec<Uuid> {}
impl QueryParamType for Vec<NaiveDate> {}
impl QueryParamType for ContactType {}
impl QueryParamType for ShirtSize {}
impl QueryParamType for SubmissionStatus {}

#[derive(Clone, Debug, PartialEq)]
pub enum QueryParam {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Uuid(Uuid),
    NaiveDate(NaiveDate),
    ArrayInt(Vec<i64>),
    ArrayFloat(Vec<f64>),
    ArrayString(Vec<String>),
    ArrayBool(Vec<bool>),
    ArrayUuid(Vec<Uuid>),
    ArrayNaiveDate(Vec<NaiveDate>),
    ContactType(ContactType),
    ShirtSize(ShirtSize),
    SubmissionStatus(SubmissionStatus),
}

impl QueryParam {
    pub fn push_bind(self, qb: &mut QueryBuilder<'_, Postgres>) {
        match self {
            QueryParam::Int(v) => qb.push_bind(v),
            QueryParam::Float(v) => qb.push_bind(v),
            QueryParam::String(v) => qb.push_bind(v),
            QueryParam::Bool(v) => qb.push_bind(v),
            QueryParam::Uuid(v) => qb.push_bind(v),
            QueryParam::NaiveDate(v) => qb.push_bind(v),
            QueryParam::ArrayInt(v) => qb.push_bind(v),
            QueryParam::ArrayFloat(v) => qb.push_bind(v),
            QueryParam::ArrayString(v) => qb.push_bind(v),
            QueryParam::ArrayBool(v) => qb.push_bind(v),
            QueryParam::ArrayUuid(v) => qb.push_bind(v),
            QueryParam::ArrayNaiveDate(v) => qb.push_bind(v),
            QueryParam::ContactType(v) => qb.push_bind(v),
            QueryParam::ShirtSize(v) => qb.push_bind(v),
            QueryParam::SubmissionStatus(v) => qb.push_bind(v),
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum QueryFragment<'sql> {
    Sql(&'sql str),
    Param(QueryParam),
    PreviousParam,
    Separator,
}

/// A trait for Queryable objects with ability to convert to query string conditions.
pub trait Queryable {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct QueryablePlaceholder;

impl Queryable for QueryablePlaceholder {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        unimplemented!("`QueryablePlaceholder` can't actually be queried");
    }
}
