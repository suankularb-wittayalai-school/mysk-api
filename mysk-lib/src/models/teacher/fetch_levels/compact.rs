use crate::{
    common::requests::FetchLevel,
    models::{
        subject_group::SubjectGroup,
        teacher::db::DbTeacher,
        traits::{FetchLevelVariant, TopLevelGetById as _},
    },
    permissions::Authorizer,
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTeacher {
    pub id: Uuid,
    pub teacher_id: Option<String>,
    pub subject_group: SubjectGroup,
}

#[async_trait]
impl FetchLevelVariant<DbTeacher> for CompactTeacher {
    async fn from_table(
        pool: &PgPool,
        table: DbTeacher,
        _: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        let subject_group =
            SubjectGroup::get_by_id(pool, table.subject_group_id, None, None, authorizer).await?;

        Ok(Self {
            id: table.id,
            teacher_id: table.teacher_id,
            subject_group,
        })
    }
}
