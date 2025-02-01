use crate::{
    common::string::MultiLangString, helpers::date::get_current_academic_year,
    models::enums::SubjectType, prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(query = "
    SELECT
        id, created_at, name_th, name_en, code_th, code_en, short_name_th, short_name_en, type,
        credit, description_th, description_en, semester, subject_group_id, syllabus
    FROM subjects
")]
pub struct DbSubject {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: String,
    pub code_th: String,
    pub code_en: String,
    pub short_name_th: Option<String>,
    pub short_name_en: Option<String>,
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
        pool: &PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT classroom_id FROM classroom_subjects WHERE subject_id = $1 AND year = $2",
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None)),
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.classroom_id).collect())
    }

    pub async fn get_subject_teachers(
        pool: &PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT teacher_id FROM subject_teachers WHERE subject_id = $1 AND year = $2",
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None)),
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.teacher_id).collect())
    }

    pub async fn get_subject_co_teachers(
        pool: &PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT teacher_id FROM subject_co_teachers WHERE subject_id = $1 AND year = $2",
            subject_id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None)),
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.teacher_id).collect())
    }

    pub async fn get_requirements(pool: &PgPool, subject_id: Uuid) -> Result<Vec<MultiLangString>> {
        let res = query!(
            "SELECT label_th, label_en FROM subject_requirements WHERE subject_id = $1",
            subject_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(res
            .into_iter()
            .map(|r| MultiLangString::new(r.label_th, r.label_en))
            .collect())
    }
}
