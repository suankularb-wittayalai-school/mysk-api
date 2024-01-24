pub mod db;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::db::DbSubjectGroup;

use super::common::{
    requests::FetchLevel,
    string::MultiLangString,
    traits::{GetById, TopLevelFromTable, TopLevelGetById},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name: MultiLangString,
}

impl TopLevelFromTable<DbSubjectGroup> for SubjectGroup {
    async fn from_table(
        _pool: &sqlx::PgPool,
        table: DbSubjectGroup,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: table.id,
            created_at: table.created_at,
            name: MultiLangString::new(table.name_th, Some(table.name_en)),
        })
    }
}

impl TopLevelGetById for SubjectGroup {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let contact = DbSubjectGroup::get_by_id(pool, id).await?;

        Self::from_table(pool, contact, _fetch_level, _descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<Uuid>,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let contacts = DbSubjectGroup::get_by_ids(pool, ids).await?;

        let mut result = vec![];

        for contact in contacts {
            result
                .push(Self::from_table(pool, contact, _fetch_level, _descendant_fetch_level).await?)
        }

        Ok(result)
    }
}
