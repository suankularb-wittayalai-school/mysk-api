use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{requests::FetchLevel, string::MultiLangString, traits::FetchLevelVariant},
    teacher::db::DbTeacher,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTeacher {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub nickname: Option<MultiLangString>,
    pub teacher_id: Option<String>,
    pub profile: Option<String>,
    pub subject_group: String, // TODO: Change to SubjectGroup
}

impl FetchLevelVariant<DbTeacher> for CompactTeacher {
    async fn from_table(
        pool: &PgPool,
        table: DbTeacher,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        // let subject_group = DbTeacher::get_teacher_subject_group(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            prefix: MultiLangString::new(table.prefix_th, table.prefix_en),
            first_name: MultiLangString::new(table.first_name_th, table.first_name_en),
            last_name: MultiLangString::new(table.last_name_th, table.last_name_en),
            nickname: table
                .nickname_th
                .map(|th| MultiLangString::new(th, table.nickname_en)),
            teacher_id: table.teacher_id,
            profile: table.profile,
            subject_group: "TODO".to_string(), // TODO: Change to SubjectGroup
        })
    }
}
