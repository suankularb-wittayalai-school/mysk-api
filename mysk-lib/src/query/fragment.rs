use crate::{
    common::requests::QueryParam,
    models::enums::{ContactType, ShirtSize, SubmissionStatus},
};
use chrono::NaiveDate;
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
impl QueryParamType for SubmissionStatus {}
impl QueryParamType for ShirtSize {}

#[derive(Debug, PartialEq)]
pub enum QueryFragment<'sql> {
    Sql(&'sql str),
    Param(QueryParam),
    PreviousParam,
    Separator,
}
