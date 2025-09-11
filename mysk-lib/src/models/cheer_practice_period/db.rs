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
use chrono::{DateTime, NaiveDate, Utc};
use futures::stream::TryStreamExt as _;
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, PgPool, Postgres, QueryBuilder, query, query_scalar};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT id, created_at, date, start_time, duration, delay FROM cheer_practice_periods",
    count_query = "SELECT COUNT(id) FROM cheer_practice_periods"
)]
pub struct DbCheerPracticePeriod {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub date: NaiveDate,
    pub start_time: i64,
    pub duration: i64,
    pub delay: Option<i64>,
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

    pub async fn get_attendance_count_by_class(
        pool: &PgPool,
        practice_period_id: Uuid,
        classroom_ids: &[Uuid],
    ) -> Result<Vec<(Uuid, i64)>> {
        let res = query!(
            r#"
            SELECT
                cs.classroom_id,
                COUNT(cpa.student_id) FILTER ( WHERE cpa.presence IN ('present', 'late') )
            FROM classroom_students cs
                JOIN cheer_practice_period_classrooms cpc on cpc.classroom_id = cs.classroom_id
                LEFT JOIN cheer_practice_attendances cpa on cpa.student_id = cs.student_id and cpa.practice_period_id = cpc.practice_period_id
            WHERE cpc.practice_period_id = $1 AND cs.classroom_id = ANY($2)
            GROUP BY cs.classroom_id ORDER BY cs.classroom_id
            "#,
            practice_period_id,
            classroom_ids,
        )
        .fetch(pool)
        .map_ok(|record| (record.classroom_id, record.count.unwrap_or(0)))
        .try_collect()
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
