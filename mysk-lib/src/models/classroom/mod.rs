use crate::models::{
    classroom::{
        db::DbClassroom,
        fetch_levels::{
            compact::CompactClassroom, default::DefaultClassroom, id_only::IdOnlyClassroom,
        },
    },
    model::Model,
};
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

pub mod db;
pub mod fetch_levels;

pub type Classroom =
    Model<DbClassroom, IdOnlyClassroom, CompactClassroom, DefaultClassroom, DefaultClassroom>;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct ClassroomWClassNo {
    pub id: Uuid,
    pub class_no: i64,
}
