use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        classroom::Classroom,
        enums::SubjectType,
        subject::db::DbSubject,
        subject_group::SubjectGroup,
        teacher::Teacher,
        traits::{FetchLevelVariant, TopLevelGetById as _},
    },
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSubject {
    pub id: Uuid,
    pub name: MultiLangString,
    pub code: MultiLangString,
    pub short_name: MultiLangString,
    pub r#type: SubjectType,
    pub credit: f64,
    pub description: Option<FlexibleMultiLangString>,
    pub semester: Option<i64>,
    pub subject_group: SubjectGroup,
    pub syllabus: Option<String>,
    pub teachers: Vec<Teacher>,
    pub co_teachers: Vec<Teacher>,
    pub classrooms: Vec<Classroom>,
}

#[async_trait]
impl FetchLevelVariant<DbSubject> for DetailedSubject {
    async fn from_table(
        pool: &PgPool,
        table: DbSubject,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &Box<dyn Authorizer>,
    ) -> Result<Self> {
        authorizer
            .authorize_subject(&table, pool, ActionType::ReadDetailed)
            .await?;

        let subject_group =
            SubjectGroup::get_by_id(pool, table.subject_group_id, None, None, authorizer).await?;

        let teacher_ids = DbSubject::get_subject_teachers(pool, table.id, None).await?;
        let co_teacher_ids = DbSubject::get_subject_co_teachers(pool, table.id, None).await?;
        let classroom_ids = DbSubject::get_subject_classrooms(pool, table.id, None).await?;

        let description = match (table.description_th, table.description_en) {
            (Some(description_th), Some(description_en)) => Some(FlexibleMultiLangString {
                th: Some(description_th),
                en: Some(description_en),
            }),
            (Some(description_th), None) => Some(FlexibleMultiLangString {
                th: Some(description_th),
                en: None,
            }),
            (None, Some(description_en)) => Some(FlexibleMultiLangString {
                th: None,
                en: Some(description_en),
            }),
            (None, None) => None,
        };

        Ok(Self {
            id: table.id,
            name: MultiLangString::new(table.name_th, Some(table.name_en)),
            code: MultiLangString::new(table.code_th, Some(table.code_en)),
            short_name: MultiLangString::new(
                table.short_name_th.unwrap_or_default(),
                table.short_name_en,
            ),
            r#type: table.r#type,
            credit: table.credit,
            description,
            semester: table.semester,
            subject_group,
            syllabus: table.syllabus,
            classrooms: Classroom::get_by_ids(
                pool,
                classroom_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            teachers: Teacher::get_by_ids(
                pool,
                teacher_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            co_teachers: Teacher::get_by_ids(
                pool,
                co_teacher_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
        })
    }
}
