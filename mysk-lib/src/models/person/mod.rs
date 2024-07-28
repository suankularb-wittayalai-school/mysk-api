use super::enums::Sex;
use crate::{common::string::MultiLangString, models::person::db::DbPerson, prelude::*};
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub mod db;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    id: Uuid,
    prefix: MultiLangString,
    first_name: MultiLangString,
    last_name: MultiLangString,
    middle_name: Option<MultiLangString>,
    nickname: Option<MultiLangString>,
}

impl Person {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self> {
        let person = DbPerson::get_by_id(pool, id).await?;

        Ok(Self {
            id: person.id,
            prefix: MultiLangString {
                th: person.prefix_th,
                en: person.prefix_en,
            },
            first_name: MultiLangString {
                th: person.first_name_th,
                en: person.first_name_en,
            },
            last_name: MultiLangString {
                th: person.last_name_th,
                en: person.last_name_en,
            },
            middle_name: person.middle_name_th.map(|th| MultiLangString {
                th,
                en: person.middle_name_en,
            }),
            nickname: person.nickname_th.map(|th| MultiLangString {
                th,
                en: person.nickname_en,
            }),
        })
    }
}
