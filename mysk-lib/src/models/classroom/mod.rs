pub mod db;
pub mod fetch_levels;

use self::{
    db::DbClassroom,
    fetch_levels::{
        compact::CompactClassroom, default::DefaultClassroom, id_only::IdOnlyClassroom,
    },
};

use serde::Deserialize;

use uuid::Uuid;

use super::common::top_level_variant::TopLevelVariant;

pub type Classroom = TopLevelVariant<
    DbClassroom,
    IdOnlyClassroom,
    CompactClassroom,
    DefaultClassroom,
    DefaultClassroom,
>;

#[derive(Debug, Clone, Deserialize, sqlx::FromRow)]
pub struct ClassroomWClassNo {
    pub id: Uuid,
    pub class_no: i64,
}
