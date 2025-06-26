use crate::{
    common::{
        requests::FetchLevel,
        string::{FlexibleMultiLangString, MultiLangString},
    },
    models::{club::db::DbClub, contact::Contact, traits::FetchVariant},
    permissions::Authorizer,
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

impl FetchVariant for DefaultClub {
    type Relation = DbClub;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let contact_ids =
            DbClub::get_club_contacts(&mut *(pool.acquire().await?), relation.id).await?;

        Ok(Self {
            id: relation.id,
            name: MultiLangString::new(relation.name_th, relation.name_en),
            description: match (relation.description_th, relation.description_en) {
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
            logo_url: relation.logo_url,
            contacts: Contact::get_by_ids(
                pool,
                &contact_ids,
                descendant_fetch_level,
                FetchLevel::IdOnly,
                authorizer,
            )
            .await?,
            accent_color: relation.accent_color,
            background_color: relation.background_color,
            member_count: relation.member_count,
            staff_count: relation.staff_count,
        })
    }
}
