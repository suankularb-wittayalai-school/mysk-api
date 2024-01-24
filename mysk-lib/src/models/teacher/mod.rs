use serde::{Deserialize, Serialize, Serializer};
use sqlx::pool;
use uuid::Uuid;

use self::{
    db::DbTeacher,
    fetch_levels::{
        compact::CompactTeacher, default::DefaultTeacher, detailed::DetailedTeacher,
        id_only::IdOnlyTeacher,
    },
};

use super::common::{
    requests::FetchLevel,
    traits::{FetchLevelVariant, GetById, TopLevelFromTable, TopLevelGetById},
};

pub mod db;
pub mod fetch_levels;

#[derive(Clone, Debug, Deserialize)]
pub enum Teacher {
    IdOnly(Box<IdOnlyTeacher>),
    Compact(Box<CompactTeacher>),
    Default(Box<DefaultTeacher>),
    Detailed(Box<DetailedTeacher>),
}

impl TopLevelFromTable<DbTeacher> for Teacher {
    async fn from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: DbTeacher,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(Box::new(table.into()))),
            Some(FetchLevel::Compact) => Ok(Self::Compact(Box::new(
                CompactTeacher::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            Some(FetchLevel::Default) => Ok(Self::Default(Box::new(
                DefaultTeacher::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(Box::new(
                DetailedTeacher::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            None => Ok(Self::IdOnly(Box::new(table.into()))),
        }
    }
}

impl Serialize for Teacher {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Teacher::IdOnly(teacher) => teacher.serialize(serializer),
            Teacher::Compact(teacher) => teacher.serialize(serializer),
            Teacher::Default(teacher) => teacher.serialize(serializer),
            Teacher::Detailed(teacher) => teacher.serialize(serializer),
        }
    }
}

impl TopLevelGetById for Teacher {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let teacher: DbTeacher = DbTeacher::get_by_id(pool, id).await?;

        Self::from_table(pool, teacher, _fetch_level, _descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<Uuid>,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let teachers = DbTeacher::get_by_ids(pool, ids).await?;

        let mut result = vec![];

        for contact in teachers {
            result
                .push(Self::from_table(pool, contact, _fetch_level, _descendant_fetch_level).await?)
        }

        Ok(result)
    }
}
