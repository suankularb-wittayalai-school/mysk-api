use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        club::db::DbClub,
        contact::Contact,
        enums::ActivityDayHouse,
        organization::Organization,
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
    pub house: Option<ActivityDayHouse>,
    pub main_room: Option<String>,
    pub map_location: Option<i64>,
}

impl FetchLevelVariant<DbClub> for DefaultClub {
    async fn from_table(
        pool: &PgPool,
        table: DbClub,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self> {
        let organization = Organization::get_by_id(pool, table.organization_id).await?;
        let contact_ids = DbClub::get_club_contacts(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            name: organization.name,
            description: organization.description,
            logo_url: organization.logo_url,
            contacts: Contact::get_by_ids(
                pool,
                contact_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
            )
            .await?,
            accent_color: table.accent_color,
            background_color: table.background_color,
            house: table.house,
            main_room: organization.main_room,
            map_location: table.map_location,
        })
    }
}
