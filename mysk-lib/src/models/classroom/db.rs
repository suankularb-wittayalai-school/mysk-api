use chrono::{DateTime, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::models::common::traits::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub struct DbClassroom {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub number: i64,
    pub year: i64,
    pub main_room: String,
}

impl BaseQuery for DbClassroom {
    fn base_query() -> &'static str {
        r#"SELECT id, created_at, number, year, main_room FROM classrooms"#
    }
}

impl GetById for DbClassroom {
    async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, DbClassroom>(format!("{} WHERE id = $1", Self::base_query()).as_str())
            .bind(id)
            .fetch_one(pool)
            .await
    }

    async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, DbClassroom>(
            format!("{} WHERE id = ANY($1)", Self::base_query()).as_str(),
        )
        .bind(ids)
        .fetch_all(pool)
        .await
    }
}

impl DbClassroom {
    pub async fn get_classroom_advisors(
        pool: &sqlx::PgPool,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        query!(
            r#"SELECT teacher_id FROM classroom_advisors WHERE classroom_id = $1"#,
            classroom_id
        )
        .fetch_all(pool)
        .await
        .map(|advisors| {
            advisors
                .into_iter()
                .map(|advisor| advisor.teacher_id)
                .collect()
        })
    }

    pub async fn get_classroom_students(
        pool: &sqlx::PgPool,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        query!(
            r#"SELECT student_id FROM classroom_students WHERE classroom_id = $1"#,
            classroom_id
        )
        .fetch_all(pool)
        .await
        .map(|students| {
            students
                .into_iter()
                .map(|student| student.student_id)
                .collect()
        })
    }

    pub async fn get_classroom_contacts(
        pool: &sqlx::PgPool,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        query!(
            r#"SELECT contact_id FROM classroom_contacts WHERE classroom_id = $1"#,
            classroom_id
        )
        .fetch_all(pool)
        .await
        .map(|contacts| {
            contacts
                .into_iter()
                .map(|contact| contact.contact_id)
                .collect()
        })
    }
}
