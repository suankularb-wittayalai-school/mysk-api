use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    classroom::db::DbClassroom,
    common::{requests::FetchLevel, traits::FetchLevelVariant},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClassroom {
    pub id: Uuid,
}

impl From<DbClassroom> for IdOnlyClassroom {
    fn from(classroom: DbClassroom) -> Self {
        Self { id: classroom.id }
    }
}

impl FetchLevelVariant<DbClassroom> for IdOnlyClassroom {
    async fn from_table(
        _pool: &PgPool,
        table: DbClassroom,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self::from(table))
    }
}
