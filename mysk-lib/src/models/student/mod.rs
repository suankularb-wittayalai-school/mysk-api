use mysk_lib_macros::traits::db::GetById;
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
    traits::{FetchLevelVariant, TopLevelFromTable, TopLevelGetById},
};

pub mod db;
pub mod fetch_levels;

#[derive(Clone, Debug, Deserialize)]
pub enum Student {
    IdOnly(Box<IdOnlyStudent>),
    Compact(Box<CompactStudent>),
    Default(Box<DefaultStudent>),
    Detailed(Box<DetailedStudent>),
}

impl TopLevelFromTable<DbStudent> for Student {
    async fn from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: DbStudent,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(Box::new(table.into()))),
            Some(FetchLevel::Compact) => Ok(Self::Compact(Box::new(CompactStudent::from(table)))),
            Some(FetchLevel::Default) => Ok(Self::Default(Box::new(
                DefaultStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(Box::new(
                DetailedStudent::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            None => Ok(Self::IdOnly(Box::new(table.into()))),
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

impl TopLevelGetById for Student {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let student = DbStudent::get_by_id(pool, id).await?;

        Self::from_table(pool, student, _fetch_level, _descendant_fetch_level).await
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
