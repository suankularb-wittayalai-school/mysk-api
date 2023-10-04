use chrono::{Date, DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub struct Person {
    pub id: Uuid,
    created_at: DateTime<Utc>,
    prefix_th: String,
    prefix_en: Option<String>,
    first_name_th: String,
    first_name_en: Option<String>,
    last_name_th: String,
    last_name_en: Option<String>,
    middle_name_th: Option<String>,
    middle_name_en: Option<String>,
    nickname_th: Option<String>,
    nickname_en: Option<String>,
    birthdate: Option<NaiveDate>,
    citizen_id: Option<String>,
    profile: Option<String>,
    pants_size: Option<String>,
    shirt_size: Option<String>,
}
