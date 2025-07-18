use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        elective_subject::db::DbElectiveSubject, enums::SubjectType, subject::db::DbSubject,
        subject_group::SubjectGroup, teacher::Teacher, traits::FetchVariant,
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

impl FetchVariant for DefaultElectiveSubject {
    type Relation = DbElectiveSubject;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        let subject_group = SubjectGroup::get_by_id(
            pool,
            relation.subject_group_id,
            FetchLevel::IdOnly,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;

        let teacher_ids =
            DbSubject::get_subject_teachers(&mut conn, relation.subject_id, None).await?;
        let co_teacher_ids =
            DbSubject::get_subject_co_teachers(&mut conn, relation.subject_id, None).await?;

        let description = match (relation.description_th, relation.description_en) {
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
            id: relation.id,
            name: MultiLangString::new(relation.name_th, Some(relation.name_en)),
            code: MultiLangString::new(relation.code_th, Some(relation.code_en)),
            short_name: MultiLangString::new(
                relation.short_name_th.unwrap_or_default(),
                relation.short_name_en,
            ),
            r#type: relation.r#type,
            credit: relation.credit,
            description,
            semester: relation.semester,
            subject_group,
            syllabus: relation.syllabus,
            teachers: Teacher::get_by_ids(
                pool,
                &teacher_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            co_teachers: Teacher::get_by_ids(
                pool,
                &co_teacher_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            class_size: relation.class_size,
            cap_size: relation.cap_size,
            room: relation.room,
            session_code: relation.session_code,
            requirements: DbSubject::get_requirements(&mut conn, relation.subject_id).await?,
        })
    }
}
