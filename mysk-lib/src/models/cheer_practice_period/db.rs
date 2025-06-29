use std::collections::HashMap;

use crate::prelude::*;
use chrono::{DateTime, NaiveDate, Utc};
use futures::stream::TryStreamExt as _;
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, PgPool, query, query_scalar};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(query = "
    SELECT id, created_at, date, start_time, duration, delay FROM cheer_practice_periods
")]
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
    ) -> Result<HashMap<Uuid, i64>> {
        let res = query!(
            "\
            SELECT c.classroom_id, COUNT(a.student_id) \
            FROM cheer_practice_attendances AS a \
                JOIN classroom_students AS c ON c.student_id = a.student_id \
            WHERE a.practice_period_id = $1 AND c.classroom_id = ANY($2) GROUP BY c.classroom_id\
            ",
            practice_period_id,
            classroom_ids,
        )
        .fetch(pool)
        .map_ok(|record| (record.classroom_id, record.count.unwrap()))
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
