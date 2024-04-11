use crate::{
    models::{
        classroom::db::DbClassroom,
        common::{requests::FetchLevel, traits::FetchLevelVariant},
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: Option<String>,
}

impl From<DbClassroom> for CompactClassroom {
    fn from(classroom: DbClassroom) -> Self {
        Self {
            id: classroom.id,
            number: classroom.number,
            room: classroom.main_room,
        }
    }
}

impl_fetch_level_variant_from!(CompactClassroom, DbClassroom);
