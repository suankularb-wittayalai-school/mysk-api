use async_trait::async_trait;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::pool;
use uuid::Uuid;

use self::{
    db::DbStudent,
    fetch_levels::{
        compact::CompactStudent, default::DefaultStudent, detailed::DetailedStudent,
        id_only::IdOnlyStudent,
    },
};

use super::common::{
    requests::FetchLevel,
    traits::{FetchLevelVariant, GetById, TopLevelFromTable, TopLevelGetById},
};

pub mod db;
pub mod fetch_levels;

#[derive(Debug, Deserialize)]
pub enum Student {
    IdOnly(IdOnlyStudent),
    Compact(Box<CompactStudent>),
    Default(Box<DefaultStudent>),
    Detailed(Box<DetailedStudent>),
}

#[async_trait::async_trait]
impl TopLevelFromTable<DbStudent> for Student {
    async fn from_table(
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
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(Box::new(
                DetailedStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            None => Ok(Self::Default(Box::new(
                DefaultStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
        }
    }
}

impl Serialize for Student {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Student::IdOnly(student) => student.serialize(serializer),
            Student::Compact(student) => student.serialize(serializer),
            Student::Default(student) => student.serialize(serializer),
            Student::Detailed(student) => student.serialize(serializer),
        }
    }
}

#[async_trait]
impl TopLevelGetById for Student {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let student = DbStudent::get_by_id(pool, id).await?;

        Ok(Self::from_table(pool, student, _fetch_level, _descendant_fetch_level).await?)
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<Uuid>,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let students = DbStudent::get_by_ids(pool, ids).await?;

        let mut result = vec![];

        for contact in students {
            result
                .push(Self::from_table(pool, contact, _fetch_level, _descendant_fetch_level).await?)
        }

        Ok(result)
    }
}