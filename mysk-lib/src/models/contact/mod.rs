use self::db::DbContact;
use crate::{
    common::{requests::FetchLevel, string::FlexibleMultiLangString},
    models::{
        enums::ContactType,
        traits::{TopLevelFromTable, TopLevelGetById},
    },
    prelude::*,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub mod db;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contact {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name: Option<FlexibleMultiLangString>,
    pub r#type: ContactType,
    pub value: String,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

#[async_trait]
impl TopLevelFromTable<DbContact> for Contact {
    async fn from_table(
        _: &PgPool,
        table: DbContact,
        _: Option<&FetchLevel>,
        _: Option<&FetchLevel>,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            created_at: table.created_at,
            name: match (table.name_th, table.name_en) {
                (Some(name_th), Some(name_en)) => Some(FlexibleMultiLangString {
                    th: Some(name_th),
                    en: Some(name_en),
                }),
                (Some(name_th), None) => Some(FlexibleMultiLangString {
                    th: Some(name_th),
                    en: None,
                }),
                (None, Some(name_en)) => Some(FlexibleMultiLangString {
                    th: None,
                    en: Some(name_en),
                }),
                (None, None) => None,
            },
            r#type: table.r#type,
            value: table.value,
            include_students: table.include_students,
            include_teachers: table.include_teachers,
            include_parents: table.include_parents,
        })
    }
}

#[async_trait]
impl TopLevelGetById for Contact {
    async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let contact = DbContact::get_by_id(pool, id).await?;

        Self::from_table(pool, contact, fetch_level, descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &PgPool,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>> {
        let contacts = DbContact::get_by_ids(pool, ids).await?;

        let mut result = vec![];

        for contact in contacts {
            result
                .push(Self::from_table(pool, contact, fetch_level, descendant_fetch_level).await?);
        }

        Ok(result)
    }
}
