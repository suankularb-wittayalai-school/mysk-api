use crate::{
    models::enums::{BloodGroup, Sex, ShirtSize},
    prelude::*,
};
use chrono::{DateTime, NaiveDate, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(
    query = "
        SELECT
            id, created_at, prefix_en, prefix_th, first_name_en, first_name_th, last_name_en, last_name_th, middle_name_en, middle_name_th, nickname_en, nickname_th, birthdate, citizen_id, profile, pants_size, shirt_size, blood_group, sex
        FROM people",
    count_query = "SELECT COUNT(distinct id) FROM people"
)]
#[get_by_id(table = "people")]
pub struct DbPerson {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub prefix_th: String,
    pub prefix_en: Option<String>,
    pub first_name_th: String,
    pub first_name_en: Option<String>,
    pub last_name_th: String,
    pub last_name_en: Option<String>,
    pub middle_name_th: Option<String>,
    pub middle_name_en: Option<String>,
    pub nickname_th: Option<String>,
    pub nickname_en: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub citizen_id: Option<String>,
    pub profile: Option<String>,
    pub shirt_size: Option<ShirtSize>,
    pub pants_size: Option<String>,
    pub blood_group: Option<BloodGroup>,
    pub sex: Sex,
}

impl DbPerson {
    pub async fn get_person_allergies(pool: &PgPool, person_id: Uuid) -> Result<Vec<String>> {
        let res = query!(
            "
            SELECT allergy_name FROM person_allergies WHERE person_id = $1
            ",
            person_id
        )
        .fetch_all(pool)
        .await?;
        match res {
            allergies => Ok(allergies.iter().map(|a| a.allergy_name.clone()).collect()),
        }
    }
}
