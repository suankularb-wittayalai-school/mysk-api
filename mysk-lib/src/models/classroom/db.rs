use crate::{helpers::date::get_current_academic_year, prelude::*};
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, query};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT id, created_at, number, year, main_room FROM classrooms")]
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
        year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT teacher_id FROM classroom_advisors JOIN classrooms AS c ON c.id = classroom_id \
            WHERE classroom_id = $1 AND year = $2\
            ",
            classroom_id,
            year.unwrap_or_else(|| get_current_academic_year(None)),
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
