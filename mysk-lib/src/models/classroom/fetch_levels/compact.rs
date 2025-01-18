use crate::{
    common::requests::FetchLevel,
    models::{classroom::db::DbClassroom, traits::FetchLevelVariant},
    permissions::{ActionType, Authorizer},
    prelude::*,
};
use async_trait::async_trait;
use mysk_lib_macros::impl_fetch_level_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactClassroom {
    pub id: Uuid,
    pub number: i64,
    pub room: Option<String>,
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

impl_fetch_level_variant_from!(classroom, Compact, CompactClassroom, DbClassroom);
