use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    common::{requests::FetchLevel, traits::FetchLevelVariant},
    student::db::DbStudent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyStudent {
    pub id: Uuid,
}

impl From<DbStudent> for IdOnlyStudent {
    fn from(student: DbStudent) -> Self {
        Self { id: student.id }
    }
}

impl FetchLevelVariant<DbStudent> for IdOnlyStudent {
    async fn from_table(
        _pool: &PgPool,
        table: DbStudent,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self::from(table))
    }
}
