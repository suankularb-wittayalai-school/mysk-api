use serde::Deserialize;
use sqlx::pool;

use self::{
    db::DbStudent,
    fetch_levels::{compact::CompactStudent, default::DefaultStudent, id_only::IdOnlyStudent},
};

use super::common::{
    requests::FetchLevel,
    traits::{FetchLevelVariant, TopLevelFromTable},
};

pub mod db;
pub mod fetch_levels;

#[derive(Debug, Deserialize)]
pub enum Student {
    IdOnly(IdOnlyStudent),
    Compact(Box<CompactStudent>),
    Default(Box<DefaultStudent>),
}

#[async_trait::async_trait]
impl TopLevelFromTable<DbStudent> for Student {
    async fn combine_from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: DbStudent,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(table.into())),
            Some(FetchLevel::Compact) => Ok(Self::Compact(Box::new(CompactStudent::from(table)))),
            Some(FetchLevel::Default) => Ok(Self::Default(Box::new(
                DefaultStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            // TODO
            Some(_) => Ok(Self::Default(Box::new(
                DefaultStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            None => Ok(Self::Default(Box::new(
                DefaultStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
        }
    }
}
