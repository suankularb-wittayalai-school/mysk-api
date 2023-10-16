use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::models::common::traits::{BaseQuery, GetById};

use super::enums::contact_type::ContactType;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub struct DbContact {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: Option<String>,
    pub name_en: Option<String>,
    pub contact_type: ContactType,
    pub value: String,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}
