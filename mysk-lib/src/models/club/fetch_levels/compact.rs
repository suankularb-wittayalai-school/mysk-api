use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{
        club::db::DbClub, enums::ActivityDayHouse, organization::Organization,
        traits::FetchLevelVariant,
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactClub {
    pub id: Uuid,
    pub name: MultiLangString,
    pub description: Option<FlexibleMultiLangString>,
    pub logo_url: Option<String>,
    pub background_color: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<i64>,
}

impl FetchLevelVariant<DbClub> for CompactClub {
    async fn from_table(
        pool: &PgPool,
        table: DbClub,
        _: Option<&FetchLevel>,
    ) -> Result<Self> {
        let organization = Organization::get_by_id(pool, table.organization_id).await?;

        Ok(Self {
            id: table.id,
            name: organization.name,
            description: organization.description,
            logo_url: organization.logo_url,
            background_color: table.background_color,
            house: table.house,
            map_location: table.map_location,
        })
    }
}
