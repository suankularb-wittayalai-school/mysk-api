use crate::models::{
    club_request::{
        db::DbClubRequest,
        fetch_levels::{default::DefaultClubRequest, id_only::IdOnlyClubRequest},
        request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
    },
    top_level_variant::TopLevelVariant,
    traits::TopLevelQuery,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ClubRequest = TopLevelVariant<
    DbClubRequest,
    IdOnlyClubRequest,
    IdOnlyClubRequest,
    DefaultClubRequest,
    DefaultClubRequest,
>;

impl TopLevelQuery<DbClubRequest, QueryableClubRequest, SortableClubRequest> for ClubRequest {}
