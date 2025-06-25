use crate::models::{
    teacher::{
        db::DbTeacher,
        fetch_levels::{
            compact::CompactTeacher, default::DefaultTeacher, detailed::DetailedTeacher,
            id_only::IdOnlyTeacher,
        },
        request::{queryable::QueryableTeacher, sortable::SortableTeacher},
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Teacher = TopLevelVariant<
    DbTeacher,
    IdOnlyTeacher,
    CompactTeacher,
    DefaultTeacher,
    DetailedTeacher,
    QueryableTeacher,
    SortableTeacher,
>;
