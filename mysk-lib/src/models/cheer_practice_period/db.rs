use std::{collections::HashSet, str::FromStr};

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

    pub async fn is_student_cheer_staff(conn: &mut PgConnection, student_id: Uuid) -> Result<bool> {
        let res = query_scalar!(
            "SELECT EXISTS (SELECT FROM cheer_practice_staffs WHERE student_id = $1)",
            student_id,
        )
        .fetch_one(conn)
        .await?
        .unwrap_or(false);

        Ok(res)
    }

    pub fn in_jaturamitr_period(practice_period_id: Uuid) -> bool {
        let jaturamitr_periods = HashSet::from([
            Uuid::from_str("870658c9-231d-454b-af62-86c0c7827ada").expect("Invalid UUID"),
            Uuid::from_str("a5b701d0-be27-4c52-a640-e23790457b61").expect("Invalid UUID"),
            Uuid::from_str("0c18a3b9-3b7f-4c71-a380-1ad0c448e35a").expect("Invalid UUID"),
        ]);

        if !jaturamitr_periods.contains(&practice_period_id) {
            false
        } else {
            true
        }
    }
}

impl QueryRelation for DbCheerPracticePeriod {
    type Q = QueryableCheerPracticePeriod;
    type S = SortableCheerPracticePeriod;

    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<Self::Q>>,
    ) {
        if let Some(filter) = filter
            && let Some(data) = filter.data
        {
            data.to_where_clause()
                .append_into_query_builder(query_builder);
        }
    }
}
