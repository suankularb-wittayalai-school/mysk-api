pub mod db;
pub mod fetch_levels;

use self::{
    db::DbClassroom,
    fetch_levels::{
        compact::CompactClassroom, default::DefaultClassroom, id_only::IdOnlyClassroom,
    },
};

use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::pool;
use uuid::Uuid;

use super::common::{
    requests::FetchLevel,
    traits::{FetchLevelVariant, TopLevelFromTable, TopLevelGetById},
};

#[derive(Clone, Debug, Deserialize)]
pub enum Classroom {
    IdOnly(Box<IdOnlyClassroom>),
    Compact(Box<CompactClassroom>),
    Default(Box<DefaultClassroom>),
    Detailed(Box<DefaultClassroom>),
}

#[derive(Debug, Clone, Deserialize, sqlx::FromRow)]
pub struct ClassroomWClassNo {
    pub id: Uuid,
    pub class_no: i64,
}

impl Serialize for Classroom {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Classroom::IdOnly(classroom) => classroom.serialize(serializer),
            Classroom::Compact(classroom) => classroom.serialize(serializer),
            Classroom::Default(classroom) => classroom.serialize(serializer),
            Classroom::Detailed(classroom) => classroom.serialize(serializer),
        }
    }
}

impl TopLevelFromTable<DbClassroom> for Classroom {
    async fn from_table(
        pool: &pool::Pool<sqlx::Postgres>,
        table: DbClassroom,
        fetch_level: Option<&FetchLevel>,
        descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(Box::new(table.into()))),
            Some(FetchLevel::Compact) => Ok(Self::Compact(Box::new(CompactClassroom::from(table)))),
            Some(FetchLevel::Default) => Ok(Self::Default(Box::new(
                Box::pin(DefaultClassroom::from_table(
                    pool,
                    table,
                    descendant_fetch_level,
                ))
                .await?,
            ))),
            Some(FetchLevel::Detailed) => Ok(Self::Detailed(Box::new(
                Box::pin(DefaultClassroom::from_table(
                    pool,
                    table,
                    descendant_fetch_level,
                ))
                .await?,
            ))),
            None => Ok(Self::IdOnly(Box::new(table.into()))),
        }
    }
}

impl TopLevelGetById for Classroom {
    async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let classroom = DbClassroom::get_by_id(pool, id).await?;

        Self::from_table(pool, classroom, _fetch_level, _descendant_fetch_level).await
    }

    async fn get_by_ids(
        pool: &sqlx::PgPool,
        ids: Vec<Uuid>,
        _fetch_level: Option<&FetchLevel>,
        _descendant_fetch_level: Option<&FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let classrooms = DbClassroom::get_by_ids(pool, ids).await?;

        let mut result = Vec::with_capacity(classrooms.len());

        for classroom in classrooms {
            result.push(
                Self::from_table(pool, classroom, _fetch_level, _descendant_fetch_level).await?,
            );
        }

        Ok(result)
    }
}
