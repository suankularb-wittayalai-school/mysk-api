use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        club::db::DbClub,
        contact::Contact,
        enums::ActivityDayHouse,
        traits::{FetchLevelVariant, TopLevelGetById},
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultClub {
    pub id: Uuid,
    pub name: MultiLangString,
    pub description: Option<FlexibleMultiLangString>,
    pub logo_url: Option<String>,
    pub contacts: Vec<Contact>,
    pub accent_color: Option<String>,
    pub background_color: Option<String>,
    pub member_count: i64,
    pub staff_count: i64,
}

impl FetchLevelVariant<DbClub> for DefaultClub {
    async fn from_table(
        pool: &PgPool,
        table: DbClub,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let contact_ids = DbClub::get_club_contacts(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            name: MultiLangString::new(table.name_th, table.name_en),
            description: match (table.description_th, table.description_en) {
                (Some(description_th), Some(description_en)) => Some(FlexibleMultiLangString {
                    th: Some(description_th),
                    en: Some(description_en),
                }),
                (Some(description_th), None) => Some(FlexibleMultiLangString {
                    th: Some(description_th),
                    en: None,
                }),
                (None, Some(description_en)) => Some(FlexibleMultiLangString {
                    th: None,
                    en: Some(description_en),
                }),
                (None, None) => None,
            },
            logo_url: table.logo_url,
            contacts: Contact::get_by_ids(
                pool,
                contact_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            accent_color: table.accent_color,
            background_color: table.background_color,
            member_count: table.member_count,
            staff_count: table.staff_count,
        })
    }
}
