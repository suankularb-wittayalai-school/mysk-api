use crate::models::{
    contact::db::DbContact,
    contact::{
        fetch_levels::{default::DefaultContact, id_only::IdOnlyContact},
        request::{queryable::QueryableContact, sortable::SortableContact},
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Contact = TopLevelVariant<
    DbContact,
    IdOnlyContact,
    IdOnlyContact,
    DefaultContact,
    DefaultContact,
    QueryableContact,
    SortableContact,
>;
