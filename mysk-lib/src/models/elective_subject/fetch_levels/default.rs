use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        elective_subject::db::DbElectiveSubject,
        enums::SubjectType,
        subject::db::DbSubject,
        subject_group::SubjectGroup,
        teacher::Teacher,
        traits::{FetchLevelVariant, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultElectiveSubject {
    pub id: Uuid,
    pub name: MultiLangString,
    pub short_name: MultiLangString,
    pub code: MultiLangString,
    pub description: Option<FlexibleMultiLangString>,
    pub teachers: Vec<Teacher>,
    pub co_teachers: Vec<Teacher>,
    pub subject_group: SubjectGroup,
    pub syllabus: Option<String>,
    pub credit: f64,
    pub class_size: i64,
    pub cap_size: i64,
    pub room: String,
    pub r#type: SubjectType,
    pub semester: Option<i64>,
    pub session_code: String,
    pub requirements: Vec<MultiLangString>,
}

impl FetchLevelVariant<DbElectiveSubject> for DefaultElectiveSubject {
    async fn from_table(
        pool: &PgPool,
        table: DbElectiveSubject,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        let subject_group =
            SubjectGroup::get_by_id(pool, table.subject_group_id, None, None, authorizer).await?;

        let teacher_ids =
            DbSubject::get_subject_teachers(&mut conn, table.subject_id, None).await?;
        let co_teacher_ids =
            DbSubject::get_subject_co_teachers(&mut conn, table.subject_id, None).await?;

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
            teachers: Teacher::get_by_ids(
                pool,
                teacher_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            co_teachers: Teacher::get_by_ids(
                pool,
                co_teacher_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            class_size: table.class_size,
            cap_size: table.cap_size,
            room: table.room,
            session_code: table.session_code,
            requirements: DbSubject::get_requirements(&mut conn, table.subject_id).await?,
        })
    }
}
