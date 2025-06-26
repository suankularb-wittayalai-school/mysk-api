use crate::{
    common::string::FlexibleMultiLangString,
    models::{contact::db::DbContact, enums::ContactType},
};
use mysk_lib_macros::impl_fetch_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultContact {
    pub id: Uuid,
    pub name: Option<FlexibleMultiLangString>,
    pub r#type: ContactType,
    pub value: String,
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
        }
    }
}

impl_fetch_variant_from!(contact, Default, DefaultContact, DbContact);
