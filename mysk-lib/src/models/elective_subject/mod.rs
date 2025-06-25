use crate::models::{
    elective_subject::{
        db::DbElectiveSubject,
        fetch_levels::{
            compact::CompactElectiveSubject, default::DefaultElectiveSubject,
            detailed::DetailedElectiveSubject, id_only::IdOnlyElectiveSubject,
        },
        request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveSubject = TopLevelVariant<
    DbElectiveSubject,
    IdOnlyElectiveSubject,
    CompactElectiveSubject,
    DefaultElectiveSubject,
    DetailedElectiveSubject,
    QueryableElectiveSubject,
    SortableElectiveSubject,
>;
