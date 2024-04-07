use chrono::{DateTime, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::prelude::*;
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
    pub semester: Option<i64>,
    pub subject_group_id: i64,
    pub syllabus: Option<String>,
}

impl DbSubject {
    pub async fn get_subject_classrooms(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT classroom_id FROM classroom_subjects WHERE subject_id = $1 AND year = $2"#,
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.classroom_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbSubject::get_subject_classrooms".to_string(),
            )),
        }
    }

    pub async fn get_subject_teachers(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT teacher_id FROM subject_teachers WHERE subject_id = $1 AND year = $2"#,
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.teacher_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbSubject::get_subject_teachers".to_string(),
            )),
        }
    }

    pub async fn get_subject_co_teachers(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT teacher_id FROM subject_co_teachers WHERE subject_id = $1 AND year = $2"#,
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.teacher_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbSubject::get_subject_co_teachers".to_string(),
            )),
        }
    }
}
