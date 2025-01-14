use crate::{
    common::requests::QueryParam,
    models::enums::{ShirtSize, SubmissionStatus},
};
use chrono::NaiveDate;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

// Additional bounds to enforce strong typing.
pub trait QueryParamType {}
impl QueryParamType for u64 {}
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
impl QueryParamType for SubmissionStatus {}
impl QueryParamType for ShirtSize {}

#[derive(Debug, PartialEq)]
pub enum QueryFragment<'sql> {
    Sql(&'sql str),
    Param(QueryParam),
    PreviousParam,
    Separator,
}

// TODO: move into `impl QueryParam`
pub fn query_param_push_bind(qb: &mut QueryBuilder<'_, Postgres>, param: QueryParam) {
    match param {
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
        QueryParam::SubmissionStatus(v) => qb.push_bind(v),
        QueryParam::ShirtSize(v) => qb.push_bind(v),
    };
}
