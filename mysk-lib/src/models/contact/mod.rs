pub mod db;
pub mod enums;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{db::DbContact, enums::contact_type::ContactType};

use super::common::{
    requests::FetchLevel,
    string::FlexibleMultiLangString,
    traits::{GetById, TopLevelFromTable, TopLevelGetById},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl TopLevelFromTable<DbContact> for Contact {
    async fn from_table(
        _pool: &sqlx::PgPool,
        table: db::DbContact,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
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

impl TopLevelGetById for Contact {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let contact = DbContact::get_by_id(pool, id).await?;

        Self::from_table(pool, contact, fetch_level, descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<Uuid>,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let contacts = DbContact::get_by_ids(pool, ids).await?;

        let mut result = vec![];

        for contact in contacts {
            result.push(Self::from_table(pool, contact, fetch_level, descendant_fetch_level).await?)
        }

        Ok(result)
    }
}
