pub mod db;
pub mod fetch_levels;

use self::{
    db::DbTeacher,
    fetch_levels::{
        compact::CompactTeacher, default::DefaultTeacher, detailed::DetailedTeacher,
        id_only::IdOnlyTeacher,
    },
};
use super::common::top_level_variant::TopLevelVariant;

pub type Teacher =
    TopLevelVariant<DbTeacher, IdOnlyTeacher, CompactTeacher, DefaultTeacher, DetailedTeacher>;
