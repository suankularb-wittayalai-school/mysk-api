use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    classroom::db::DbClassroom,
    common::{requests::FetchLevel, traits::FetchLevelVariant},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: String,
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

impl FetchLevelVariant<DbClassroom> for CompactClassroom {
    async fn from_table(
        _pool: &PgPool,
        table: DbClassroom,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self::from(table))
    }
}
