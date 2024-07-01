use self::db::DbSubjectGroup;
use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::traits::TopLevelFromTable,
    prelude::*,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod db;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubjectGroup {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name: MultiLangString,
}

#[async_trait]
impl TopLevelFromTable<DbSubjectGroup> for SubjectGroup {
    async fn from_table(
        _: &PgPool,
        table: DbSubjectGroup,
        _: Option<&FetchLevel>,
        _: Option<&FetchLevel>,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            created_at: table.created_at,
            name: MultiLangString::new(table.name_th, Some(table.name_en)),
        })
    }
}

// We're not using the GetById derive here because the ID of this table is an integer not a UUID.
impl SubjectGroup {
    pub async fn get_by_id(
        pool: &PgPool,
        id: i64,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let contact = DbSubjectGroup::get_by_id(pool, id).await?;

        Self::from_table(pool, contact, fetch_level, descendant_fetch_level).await
    }

    pub async fn get_by_ids(
        pool: &PgPool,
        ids: Vec<i64>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>> {
        let contacts = DbSubjectGroup::get_by_ids(pool, ids).await?;
        let fetch_level = fetch_level.copied();
        let descendant_fetch_level = descendant_fetch_level.copied();
        let futures: Vec<_> = contacts
            .into_iter()
            .map(|contact| {
                let pool = pool.clone();

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        contact,
                        fetch_level.as_ref(),
                        descendant_fetch_level.as_ref(),
                    )
                    .await
                })
            })
            .collect();

        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            result.push(future.await.unwrap()?);
        }

        Ok(result)
    }
}
