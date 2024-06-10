use self::{
    db::DbClub,
    fetch_levels::{
        compact::CompactClub, default::DefaultClub, detailed::DetailedClub, id_only::IdOnlyClub,
    },
    request::{queryable::QueryableClub, sortable::SortableClub},
};
use crate::models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Club = TopLevelVariant<DbClub, IdOnlyClub, CompactClub, DefaultClub, DetailedClub>;

impl TopLevelQuery<DbClub, QueryableClub, SortableClub> for Club {}
