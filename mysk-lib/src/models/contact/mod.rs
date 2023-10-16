pub mod db;
pub mod enums;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use self::{db::DbContact, enums::contact_type::ContactType};

use super::common::{
    requests::FetchLevel, string::FlexibleMultiLangString, traits::TopLevelFromTable,
};

#[derive(Debug, Clone, serde::Deserialize)]
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
