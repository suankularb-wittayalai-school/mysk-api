use chrono::{DateTime, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::prelude::*;
use crate::{
    helpers::date::get_current_academic_year, models::subject::enums::subject_type::SubjectType,
};

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
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

impl DbElectiveSubject {
    pub async fn get_subject_applicable_classrooms(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT classroom_id FROM elective_subject_classrooms WHERE elective_subject_id = $1"#,
            self.id
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.classroom_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_subject_applicable_classrooms".to_string(),
            )),
        }
    }

    pub async fn get_enrolled_students(
        &self,
        pool: &sqlx::PgPool,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            r#"SELECT student_id FROM student_elective_subjects WHERE elective_subject_id = $1 and year = $2"#,
            self.id,
            academic_year.unwrap_or_else(|| get_current_academic_year(None))
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.student_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbElectiveSubject::get_enrolled_students".to_string(),
            )),
        }
    }
}
