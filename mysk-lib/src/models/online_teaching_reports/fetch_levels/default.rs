use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom, online_teaching_reports::db::DbOnlineTeachingReports,
        subject::Subject, teacher::Teacher, traits::FetchLevelVariant,
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultOnlineTeachingReports {
    pub id: Uuid,
    pub subject: Option<Subject>,
    pub teacher: Teacher,
    pub classroom: Option<Classroom>,
    pub date: NaiveDate,
    pub teaching_methods: Vec<String>,
    pub teaching_topic: String,
    pub suggestions: Option<String>,
    pub absent_student_no: Option<String>,
    pub start_time: i64,
    pub duration: i64,
    pub has_image: bool,
}

impl FetchLevelVariant<DbOnlineTeachingReports> for DefaultOnlineTeachingReports {
    async fn from_table(
        pool: &PgPool,
        table: DbOnlineTeachingReports,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_online_teaching_reports(
                &table,
                &mut *(pool.acquire().await?),
                ActionType::ReadDefault,
            )
            .await?;

        let subject = if table.subject_id.is_some() {
            Some(
                Subject::get_by_id(
                    pool,
                    table.subject_id.unwrap(),
                    descendant_fetch_level,
                    FetchLevel::IdOnly,
                    authorizer,
                )
                .await?,
            )
        } else {
            None
        };
        let classroom = if table.classroom_id.is_some() {
            Some(
                Classroom::get_by_id(
                    pool,
                    table.classroom_id.unwrap(),
                    descendant_fetch_level,
                    FetchLevel::IdOnly,
                    authorizer,
                )
                .await?,
            )
        } else {
            None
        };

        Ok(Self {
            id: table.id,
            subject,
            teacher: Teacher::get_by_id(
                pool,
                table.teacher_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            classroom,
            date: table.date,
            teaching_methods: table.teaching_methods,
            teaching_topic: table.teaching_topic,
            suggestions: table.suggestions,
            absent_student_no: table.absent_student_no,
            start_time: table.start_time,
            duration: table.duration,
            has_image: table.has_image,
        })
    }
}
