use crate::{
    common::requests::FetchLevel,
    models::{subject_group::SubjectGroup, teacher::db::DbTeacher, traits::FetchVariant},
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTeacher {
    pub id: Uuid,
    pub teacher_id: Option<String>,
    pub subject_group: SubjectGroup,
}

impl FetchVariant for CompactTeacher {
    type Relation = DbTeacher;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        _: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let subject_group = SubjectGroup::get_by_id(
            pool,
            relation.subject_group_id,
            FetchLevel::IdOnly,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;

        Ok(Self {
            id: relation.id,
            teacher_id: relation.teacher_id,
            subject_group,
        })
    }
}
