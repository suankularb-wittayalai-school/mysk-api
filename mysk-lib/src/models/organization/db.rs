use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(query = "SELECT * FROM organizations")]
pub struct DbOrganization {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: Option<String>,
    pub description_th: Option<String>,
    pub description_en: Option<String>,
    pub main_room: Option<String>,
    pub logo_url: Option<String>,
    pub user_id: Option<Uuid>,
}
