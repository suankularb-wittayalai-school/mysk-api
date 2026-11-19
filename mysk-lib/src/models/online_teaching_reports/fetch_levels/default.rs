use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom, online_teaching_reports::db::DbOnlineTeachingReports,
        subject::Subject, teacher::Teacher, traits::FetchVariant,
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

impl FetchVariant for DefaultOnlineTeachingReports {
    type Relation = DbOnlineTeachingReports;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_online_teaching_reports(
                &relation,
                &mut *(pool.acquire().await?),
                ActionType::ReadDefault,
            )
            .await?;

        let subject = if let Some(subject_id) = relation.subject_id {
            Some(
                Subject::get_by_id(
                    pool,
                    subject_id,
                    descendant_fetch_level,
                    FetchLevel::IdOnly,
                    authorizer,
                )
                .await?,
            )
        } else {
            None
        };

        let classroom = if let Some(clasroom_id) = relation.classroom_id {
            Some(
                Classroom::get_by_id(
                    pool,
                    clasroom_id,
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
            id: relation.id,
            subject,
            teacher: Teacher::get_by_id(
                pool,
                relation.teacher_id,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            classroom,
            date: relation.date,
            teaching_methods: relation.teaching_methods,
            teaching_topic: relation.teaching_topic,
            suggestions: relation.suggestions,
            absent_student_no: relation.absent_student_no,
            start_time: relation.start_time,
            duration: relation.duration,
            has_image: relation.has_image,
        })
    }
}
