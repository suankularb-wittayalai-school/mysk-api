use crate::models::{
    student::{
        db::DbStudent,
        fetch_levels::{compact::CompactStudent, default::DefaultStudent, id_only::IdOnlyStudent},
        request::{queryable::QueryableStudent, sortable::SortableStudent},
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Student = TopLevelVariant<
    DbStudent,
    IdOnlyStudent,
    CompactStudent,
    DefaultStudent,
    DefaultStudent,
    QueryableStudent,
    SortableStudent,
>;
