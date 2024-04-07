use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::subject::db::DbSubject;
use crate::models::subject::enums::subject_type::SubjectType;
use crate::prelude::*;

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT * FROM complete_elective_subjects_view")]
pub struct DbElectiveSubject {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub subject_id: Uuid,
    pub cap_size: i64,
    pub class_size: i64,
    pub room: String,
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

impl DbElectiveSubject {
    pub async fn get_subject_classrooms(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        DbSubject::get_subject_classrooms(pool, subject_id, academic_year).await
    }

    pub async fn get_subject_teachers(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        DbSubject::get_subject_teachers(pool, subject_id, academic_year).await
    }

    pub async fn get_subject_co_teachers(
        pool: &sqlx::PgPool,
        subject_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        DbSubject::get_subject_co_teachers(pool, subject_id, academic_year).await
    }
}
