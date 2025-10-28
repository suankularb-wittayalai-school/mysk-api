use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        enums::SubjectType, subject::db::DbSubject, subject_group::SubjectGroup, teacher::Teacher,
        traits::FetchVariant,
    },
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSubject {
    pub id: Uuid,
    pub name: MultiLangString,
    pub code: MultiLangString,
    pub short_name: MultiLangString,
    pub r#type: SubjectType,
    pub credit: f64,
    pub description: Option<FlexibleMultiLangString>,
    pub semester: Option<i64>,
    pub subject_group: SubjectGroup,
    pub teachers: Vec<Teacher>,
    pub co_teachers: Vec<Teacher>,
    pub syllabus: Option<String>,
}

impl FetchVariant for DefaultSubject {
    type Relation = DbSubject;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let mut conn = pool.acquire().await?;
        authorizer
            .authorize_subject(&relation, &mut conn, ActionType::ReadDefault)
            .await?;

        let subject_group = SubjectGroup::get_by_id(
            pool,
            relation.subject_group_id,
            FetchLevel::IdOnly,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;
        let teacher_ids = DbSubject::get_subject_teachers(&mut conn, relation.id, None).await?;
        let co_teacher_ids =
            DbSubject::get_subject_co_teachers(&mut conn, relation.id, None).await?;
        drop(conn);

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
        })
    }
}
