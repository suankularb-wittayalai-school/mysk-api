use crate::models::{
    contact::db::DbContact,
    contact::{
        fetch_levels::{default::DefaultContact, id_only::IdOnlyContact},
        request::{queryable::QueryableContact, sortable::SortableContact},
    },
    top_level_variant::TopLevelVariant,
    traits::TopLevelQuery,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Contact =
    TopLevelVariant<DbContact, IdOnlyContact, IdOnlyContact, DefaultContact, DefaultContact>;

impl TopLevelQuery<DbContact, QueryableContact, SortableContact> for Contact {}
