use super::enums::{Sex, ShirtSize};
use crate::{common::string::MultiLangString, models::person::db::DbPerson, prelude::*};
use chrono::NaiveDate;
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub mod db;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub nickname: Option<MultiLangString>,
    pub birthdate: Option<NaiveDate>,
    pub allergies: Vec<String>,
    pub shirt_size: Option<ShirtSize>,
    pub pants_size: Option<String>,
    pub sex: Sex,
}

impl Person {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self> {
        let person = DbPerson::get_by_id(pool, id).await?;

        Ok(Self {
            id: person.id,
            prefix: MultiLangString::new(person.prefix_th, person.prefix_en),
            first_name: MultiLangString::new(person.first_name_th, person.first_name_en),
            last_name: MultiLangString::new(person.last_name_th, person.last_name_en),
            middle_name: person
                .middle_name_th
                .map(|th| MultiLangString::new(th, person.middle_name_en)),
            nickname: person
                .nickname_th
                .map(|th| MultiLangString::new(th, person.nickname_en)),
            birthdate: person.birthdate,
            allergies: DbPerson::get_person_allergies(pool, person.id).await?,
            shirt_size: person.shirt_size,
            pants_size: person.pants_size,
            sex: person.sex,
        })
    }
}
