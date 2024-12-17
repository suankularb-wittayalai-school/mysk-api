use crate::models::{
    student::{
        db::DbStudent,
        fetch_levels::{compact::CompactStudent, default::DefaultStudent, id_only::IdOnlyStudent},
        request::{queryable::QueryableStudent, sortable::SortableStudent},
    },
    top_level_variant::TopLevelVariant,
    traits::TopLevelQuery,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Student =
    TopLevelVariant<DbStudent, IdOnlyStudent, CompactStudent, DefaultStudent, DefaultStudent>;

impl TopLevelQuery<DbStudent, QueryableStudent, SortableStudent> for Student {}
