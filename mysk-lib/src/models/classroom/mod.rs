use self::{
    db::DbClassroom,
    fetch_levels::{
        compact::CompactClassroom, default::DefaultClassroom, id_only::IdOnlyClassroom,
    },
};
use super::common::top_level_variant::TopLevelVariant;
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

pub mod db;
pub mod fetch_levels;

pub type Classroom = TopLevelVariant<
    DbClassroom,
    IdOnlyClassroom,
    CompactClassroom,
    DefaultClassroom,
    DefaultClassroom,
>;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct ClassroomWClassNo {
    pub id: Uuid,
    pub class_no: i64,
}
