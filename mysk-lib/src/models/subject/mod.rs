use serde::{Deserialize, Serialize, Serializer};
use sqlx::PgPool;
use uuid::Uuid;

use self::{
    db::DbSubject,
    fetch_levels::{
        compact::CompactSubject, default::DefaultSubject, detailed::DetailedSubject,
        id_only::IdOnlySubject,
    },
};

use super::common::{
    requests::FetchLevel,
    traits::{FetchLevelVariant, TopLevelFromTable, TopLevelGetById},
};

pub mod db;
pub mod enums;
pub mod fetch_levels;

#[derive(Clone, Debug, Deserialize)]
pub enum Subject {
    IdOnly(Box<IdOnlySubject>),
    Compact(Box<CompactSubject>),
    Default(Box<DefaultSubject>),
    Detailed(Box<DetailedSubject>),
}

impl TopLevelFromTable<DbSubject> for Subject {
    async fn from_table(
        pool: &PgPool,
        table: DbSubject,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(Box::new(table.into()))),
            Some(FetchLevel::Compact) => Ok(Self::Compact(Box::new(CompactSubject::from(table)))),
            Some(FetchLevel::Default) => Ok(Self::Default(Box::new(
                DefaultSubject::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(Box::new(
                DetailedSubject::from_table(pool, table, descendant_fetch_level).await?,
            ))),
            None => Ok(Self::IdOnly(Box::new(table.into()))),
        }
    }
}

impl Serialize for Subject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Subject::IdOnly(subject) => subject.serialize(serializer),
            Subject::Compact(subject) => subject.serialize(serializer),
            Subject::Default(subject) => subject.serialize(serializer),
            Subject::Detailed(subject) => subject.serialize(serializer),
        }
    }
}

impl TopLevelGetById for Subject {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let subject = DbSubject::get_by_id(pool, id).await?;

        Self::from_table(pool, subject, _fetch_level, _descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<Uuid>,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let subjects = DbSubject::get_by_ids(pool, ids).await?;

        let mut result = vec![];

        for contact in subjects {
            result
                .push(Self::from_table(pool, contact, _fetch_level, _descendant_fetch_level).await?)
        }

        Ok(result)
    }
}
