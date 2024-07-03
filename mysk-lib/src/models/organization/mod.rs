use crate::{
    common::string::{FlexibleMultiLangString, MultiLangString},
    models::organization::db::DbOrganization,
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::traits::db::GetById as _;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub mod db;

#[derive(Clone, Debug, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name: MultiLangString,
    pub description: Option<FlexibleMultiLangString>,
    pub main_room: Option<String>,
    pub logo_url: Option<String>,
    pub user_id: Option<Uuid>,
}

impl Organization {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self> {
        let organization = DbOrganization::get_by_id(pool, id).await?;

        Ok(Self {
            id: organization.id,
            created_at: organization.created_at,
            name: MultiLangString::new(organization.name_th, organization.name_en),
            description: match (organization.description_th, organization.description_en) {
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
            main_room: organization.main_room,
            logo_url: organization.logo_url,
            user_id: organization.user_id,
        })
    }
}
