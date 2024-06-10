use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{club::db::DbClub, traits::FetchLevelVariant},
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
    pub member_count: i64,
}

impl From<DbClub> for CompactClub {
    fn from(club: DbClub) -> Self {
        Self {
            id: club.id,
            name: MultiLangString::new(club.name_th, club.name_en),
            description: match (club.description_th, club.description_en) {
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
            logo_url: club.logo_url,
            background_color: club.background_color,
            member_count: club.member_count,
        }
    }
}

impl_fetch_level_variant_from!(CompactClub, DbClub);
