use self::{
    db::DbStudent,
    fetch_levels::{
        compact::CompactStudent, default::DefaultStudent, detailed::DetailedStudent,
        id_only::IdOnlyStudent,
    },
};

use super::common::top_level_variant::TopLevelVariant;

pub mod db;
pub mod fetch_levels;

pub type Student =
    TopLevelVariant<DbStudent, IdOnlyStudent, CompactStudent, DefaultStudent, DetailedStudent>;
