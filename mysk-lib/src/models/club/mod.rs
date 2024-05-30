use self::{
    db::DbClub,
    fetch_levels::{
        compact::CompactClub, default::DefaultClub, detailed::DetailedClub, id_only::IdOnlyClub,
    },
};
use crate::{
    common::requests::{QueryablePlaceholder, SortablePlaceholder},
    models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery as _},
};

pub mod db;
pub mod fetch_levels;

pub type Club = TopLevelVariant<DbClub, IdOnlyClub, CompactClub, DefaultClub, DetailedClub>;

// TODO: Queryable and Sortable to be implemented
impl TopLevelQuery<DbClub, QueryablePlaceholder, SortablePlaceholder> for Club {}
