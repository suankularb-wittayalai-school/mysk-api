use crate::{
    common::string::MultiLangString,
    models::{
        enums::{Sex, ShirtSize},
        person::db::DbPerson,
        traits::GetById as _,
    },
    prelude::*,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
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
    pub async fn get_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Self> {
        let person = DbPerson::get_by_id(conn, id).await?;

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
            allergies: DbPerson::get_person_allergies(conn, person.id).await?,
            shirt_size: person.shirt_size,
            pants_size: person.pants_size,
            sex: person.sex,
        })
    }
}
