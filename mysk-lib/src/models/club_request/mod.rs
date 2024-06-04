use self::{
    db::DbClubRequest,
    fetch_levels::default::DefaultClubRequest,
    request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
};

use crate::models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};
pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ClubRequest = TopLevelVariant<
    DbClubRequest,
    IdOnlyClubRequest,
    CompactClubRequest,
    DefaultClubRequest,
    DetailedClubRequest,
>;

impl TopLevelQuery<DbClubRequest, QueryableClubRequest, SortableClubRequest> for ClubRequest {}
