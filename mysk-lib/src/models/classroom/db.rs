use crate::{
    common::requests::FilterConfig,
    models::{
        classroom::request::{queryable::QueryableClassroom, sortable::SortableClassroom},
        traits::QueryRelation,
    },
    prelude::*,
    query::Queryable as _,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, Postgres, QueryBuilder, query};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT id, created_at, number, year, main_room FROM classrooms",
    count_query = "SELECT COUNT(*) FROM classrooms"
)]
pub struct DbClassroom {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub number: i64,
    pub year: i64,
    pub main_room: Option<String>,
}

impl DbClassroom {
    pub async fn get_classroom_advisors(
        conn: &mut PgConnection,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT teacher_id FROM classroom_advisors WHERE classroom_id = $1",
            classroom_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|advisor| advisor.teacher_id).collect())
    }

    pub async fn get_classroom_students(
        conn: &mut PgConnection,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT student_id FROM classroom_students WHERE classroom_id = $1",
            classroom_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|student| student.student_id).collect())
    }

    pub async fn get_classroom_contacts(
        conn: &mut PgConnection,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT contact_id FROM classroom_contacts WHERE classroom_id = $1",
            classroom_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|contact| contact.contact_id).collect())
    }
}

impl QueryRelation for DbClassroom {
    type Q = QueryableClassroom;
    type S = SortableClassroom;

    fn build_shared_query(
        query_builder: &mut QueryBuilder<Postgres>,
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
