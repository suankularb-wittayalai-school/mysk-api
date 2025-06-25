use crate::models::{
    club::{
        db::DbClub,
        fetch_levels::{
            compact::CompactClub, default::DefaultClub, detailed::DetailedClub, id_only::IdOnlyClub,
        },
        request::{queryable::QueryableClub, sortable::SortableClub},
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Club = TopLevelVariant<
    DbClub,
    IdOnlyClub,
    CompactClub,
    DefaultClub,
    DetailedClub,
    QueryableClub,
    SortableClub,
>;
