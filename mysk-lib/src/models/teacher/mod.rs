use self::{
    db::DbTeacher,
    fetch_levels::{
        compact::CompactTeacher, default::DefaultTeacher, detailed::DetailedTeacher,
        id_only::IdOnlyTeacher,
    },
};
use crate::models::top_level_variant::TopLevelVariant;

pub mod db;
pub mod fetch_levels;

pub type Teacher =
    TopLevelVariant<DbTeacher, IdOnlyTeacher, CompactTeacher, DefaultTeacher, DetailedTeacher>;
