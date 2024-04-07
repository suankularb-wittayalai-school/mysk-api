use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use sqlx::query;
use uuid::Uuid;

use crate::prelude::*;
use crate::{
    helpers::date::get_current_academic_year,
    // models::common::traits::{BaseQuery, GetById},
};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
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
        pool: &sqlx::PgPool,
        classroom_id: Uuid,
        year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT teacher_id FROM classroom_advisors INNER JOIN classrooms ON classrooms.id = classroom_id WHERE classroom_id = $1 AND year = $2"#,
            classroom_id,
            year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await
        .map(|advisors| {
            advisors
                .into_iter()
                .map(|advisor| advisor.teacher_id)
                .collect()
        });

        match res {
            Ok(advisors) => Ok(advisors),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClassroom::get_classroom_advisors".to_string(),
            )),
        }
    }

    pub async fn get_classroom_students(
        pool: &sqlx::PgPool,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
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
        });

        match res {
            Ok(students) => Ok(students),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClassroom::get_classroom_students".to_string(),
            )),
        }
    }

    pub async fn get_classroom_contacts(
        pool: &sqlx::PgPool,
        classroom_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
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
        });

        match res {
            Ok(contacts) => Ok(contacts),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClassroom::get_classroom_contacts".to_string(),
            )),
        }
    }
}
