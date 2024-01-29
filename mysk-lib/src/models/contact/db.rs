use chrono::{DateTime, Utc};
use uuid::Uuid;

use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};

// use crate::models::common::traits::{BaseQuery, GetById};

use super::enums::contact_type::ContactType;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT id, created_at, name_th, name_en, type, value, include_students, include_teachers, include_parents FROM contacts"
)]
pub struct DbContact {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: Option<String>,
    pub name_en: Option<String>,
    pub r#type: ContactType,
    pub value: String,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}
