use crate::{
    common::requests::FilterConfig,
    models::{
        cheer_practice_period::request::{
            queryable::QueryableCheerPracticePeriod, sortable::SortableCheerPracticePeriod,
        },
        traits::QueryRelation,
    },
    prelude::*,
    query::Queryable as _,
};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, Postgres, QueryBuilder, query_scalar};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT id, created_at, date, start_time, end_time, delay, note FROM cheer_practice_periods",
    count_query = "SELECT COUNT(id) FROM cheer_practice_periods"
)]
pub struct DbCheerPracticePeriod {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub delay: Option<i64>,
    pub note: Option<String>,
}

impl DbCheerPracticePeriod {
    pub async fn get_classroom_ids(conn: &mut PgConnection, id: Uuid) -> Result<Vec<Uuid>> {
        let res = query_scalar!(
            "\
            SELECT classroom_id \
            FROM cheer_practice_period_classrooms WHERE practice_period_id = $1 \
            ORDER BY classroom_id\
            ",
            id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res)
    }

    pub async fn get_cheer_staffs(conn: &mut PgConnection) -> Result<Vec<Uuid>> {
        let res = query_scalar!("SELECT student_id FROM cheer_practice_staffs ORDER BY id")
            .fetch_all(conn)
            .await?;

        Ok(res)
    }
}

impl QueryRelation for DbCheerPracticePeriod {
    type Q = QueryableCheerPracticePeriod;
    type S = SortableCheerPracticePeriod;

    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<Self::Q>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
