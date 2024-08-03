use crate::{
    common::{requests::FetchLevel, string::FlexibleMultiLangString},
    models::{contact::db::DbContact, enums::ContactType, traits::FetchLevelVariant},
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use mysk_lib_macros::impl_fetch_level_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultContact {
    pub id: Uuid,
    pub name: Option<FlexibleMultiLangString>,
    pub r#type: ContactType,
    pub value: String,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

impl From<DbContact> for DefaultContact {
    fn from(contact: DbContact) -> Self {
        Self {
            id: contact.id,
            name: match (contact.name_th, contact.name_en) {
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
            r#type: contact.r#type,
            value: contact.value,
            include_students: contact.include_students,
            include_teachers: contact.include_teachers,
            include_parents: contact.include_parents,
        }
    }
}

impl_fetch_level_variant_from!(contact, Default, DefaultContact, DbContact);
