use serde::Deserialize;
use sqlx::pool;

use self::{
    db::DbStudent,
    fetch_levels::{compact::CompactStudent, id_only::IdOnlyStudent},
};

use super::common::{requests::FetchLevel, traits::CombineFromTable};

pub mod db;
pub mod fetch_levels;

#[derive(Debug, Deserialize)]
pub enum Student {
    IdOnly(IdOnlyStudent),
    CompactStudent(Box<CompactStudent>),
}

#[async_trait::async_trait]
impl CombineFromTable<DbStudent> for Student {
    async fn combine_from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: DbStudent,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(table.into())),
            Some(FetchLevel::Compact) => {
                Ok(Self::CompactStudent(Box::new(CompactStudent::from(table))))
            }
            // TODO
            Some(_) => Ok(Self::CompactStudent(Box::new(CompactStudent::from(table)))),
            None => Ok(Self::CompactStudent(Box::new(CompactStudent::from(table)))),
        }
    }
}
