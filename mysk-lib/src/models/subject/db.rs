use chrono::{DateTime, Utc};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

use crate::{
    helpers::date::get_current_academic_year,
    // models::common::traits::{BaseQuery, GetById},
};

use super::enums::subject_type::SubjectType;

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT id, created_at, name_th, name_en, code_th, code_en, short_name_th, short_name_en, type, credit, description_th, description_en, semester, subject_group_id, syllabus FROM subjects"
)]
pub struct DbSubject {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: String,
    pub code_th: String,
    pub code_en: String,
    pub short_name_th: String,
    pub short_name_en: String,
    pub r#type: SubjectType,
    pub credit: f64,
    pub description_th: Option<String>,
    pub description_en: Option<String>,
    pub semester: i64,
    pub subject_group_id: i64,
    pub syllabus: Option<String>,
}

// impl BaseQuery for DbSubject {
//     fn base_query() -> &'static str {
//         r#"SELECT id, created_at, name_th, name_en, code_th, code_en, short_name_th, short_name_en, type, credit, description_th, description_en, semester, subject_group_id, syllabus FROM subjects"#
//     }
// }

// impl GetById for DbSubject {
//     async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
//         sqlx::query_as::<_, DbSubject>(format!("{} WHERE id = $1", Self::base_query()).as_str())
//             .bind(id)
//             // sqlx::query_as!(DbContact, r#"SELECT id, created_at, name_th, name_en, type as "type: _", value, include_students, include_teachers, include_parents FROM contacts WHERE id = $1"#, id)
//             .fetch_one(pool)
//             .await
//     }

//     async fn get_by_ids(pool: &sqlx::PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>, sqlx::Error> {
//         sqlx::query_as::<_, DbSubject>(
//             format!("{} WHERE id = ANY($1)", Self::base_query()).as_str(),
//         )
//         .bind(ids)
//         .fetch_all(pool)
//         .await
//     }
// }

impl DbSubject {
    pub async fn get_subject_classrooms(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT classroom_id FROM classroom_subjects WHERE subject_id = $1 AND year = $2"#,
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.classroom_id).collect())
    }

    pub async fn get_subject_teachers(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT teacher_id FROM subject_teachers WHERE subject_id = $1 AND year = $2"#,
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.teacher_id).collect())
    }

    pub async fn get_subject_co_teachers(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let res = query!(
            r#"SELECT teacher_id FROM subject_co_teachers WHERE subject_id = $1 AND year = $2"#,
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.teacher_id).collect())
    }
}
