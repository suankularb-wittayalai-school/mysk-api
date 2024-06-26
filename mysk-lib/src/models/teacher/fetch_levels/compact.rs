use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::{subject_group::SubjectGroup, teacher::db::DbTeacher, traits::FetchLevelVariant},
    prelude::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTeacher {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub teacher_id: Option<String>,
    pub profile_url: Option<String>,
    pub subject_group: SubjectGroup,
}

#[async_trait]
impl FetchLevelVariant<DbTeacher> for CompactTeacher {
    async fn from_table(pool: &PgPool, table: DbTeacher, _: Option<&FetchLevel>) -> Result<Self> {
        let subject_group =
            SubjectGroup::get_by_id(pool, table.subject_group_id, None, None).await?;

        Ok(Self {
            id: table.id,
            prefix: MultiLangString::new(table.prefix_th, table.prefix_en),
            first_name: MultiLangString::new(table.first_name_th, table.first_name_en),
            last_name: MultiLangString::new(table.last_name_th, table.last_name_en),
            nickname: table
                .nickname_th
                .map(|th| MultiLangString::new(th, table.nickname_en)),
            teacher_id: table.teacher_id,
            profile_url: table.profile,
            subject_group,
        })
    }
}
