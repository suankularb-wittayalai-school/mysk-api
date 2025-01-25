use crate::{
    common::requests::FetchLevel,
    models::{
        classroom::Classroom,
        online_teaching_reports::db::DbOnlineTeachingReports,
        subject::Subject,
        teacher::Teacher,
        traits::{FetchLevelVariant, TopLevelGetById},
    },
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultOnlineTeachingReports {
    pub id: Uuid,
    pub subject: Subject,
    pub teacher: Teacher,
    pub classroom: Classroom,
    pub date: NaiveDate,
    pub teaching_methods: Vec<String>,
    pub teaching_topic: String,
    pub suggestions: Option<String>,
}

#[async_trait]
impl FetchLevelVariant<DbOnlineTeachingReports> for DefaultOnlineTeachingReports {
    async fn from_table(
        pool: &PgPool,
        table: DbOnlineTeachingReports,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_online_teaching_reports(&table, pool, ActionType::ReadDefault)
            .await?;

        Ok(Self {
            id: table.id,
            subject: Subject::get_by_id(
                pool,
                table.subject_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            teacher: Teacher::get_by_id(
                pool,
                table.teacher_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            classroom: Classroom::get_by_id(
                pool,
                table.classroom_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            date: table.date,
            teaching_methods: table.teaching_methods,
            teaching_topic: table.teaching_topic,
            suggestions: table.suggestions,
        })
    }
}
