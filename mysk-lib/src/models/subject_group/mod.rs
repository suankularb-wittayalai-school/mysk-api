use crate::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::{
        subject_group::db::DbSubjectGroup,
        traits::{TopLevelFromTable, TopLevelGetById},
    },
    permissions::Authorizer,
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
        _: &Box<dyn Authorizer>,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            created_at: table.created_at,
            name: MultiLangString::new(table.name_th, Some(table.name_en)),
        })
    }
}

// We're not using the GetById derive here because the ID of this table is an integer not a UUID.
#[async_trait]
impl TopLevelGetById for SubjectGroup {
    type Id = i64;

    async fn get_by_id(
        pool: &PgPool,
        id: Self::Id,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &Box<dyn Authorizer>,
    ) -> Result<Self> {
        let contact = DbSubjectGroup::get_by_id(pool, id).await?;

        Self::from_table(
            pool,
            contact,
            fetch_level,
            descendant_fetch_level,
            authorizer,
        )
        .await
    }

    async fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Self::Id>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &Box<dyn Authorizer>,
    ) -> Result<Vec<Self>> {
        let contacts = DbSubjectGroup::get_by_ids(pool, ids).await?;
        let fetch_level = fetch_level.copied();
        let descendant_fetch_level = descendant_fetch_level.copied();
        let futures: Vec<_> = contacts
            .into_iter()
            .map(|contact| {
                let pool = pool.clone();
                let authorizer = dyn_clone::clone_box(&**authorizer);

                tokio::spawn(async move {
                    Self::from_table(
                        &pool,
                        contact,
                        fetch_level.as_ref(),
                        descendant_fetch_level.as_ref(),
                        &authorizer,
                    )
                    .await
                })
            })
            .collect();

        let mut result = Vec::with_capacity(futures.len());
        for future in futures {
            result.push(future.await??);
        }

        Ok(result)
    }
}
