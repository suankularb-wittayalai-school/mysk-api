use crate::models::{
    club_request::{
        db::DbClubRequest,
        fetch_levels::{default::DefaultClubRequest, id_only::IdOnlyClubRequest},
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ClubRequest = Model<
    DbClubRequest,
    IdOnlyClubRequest,
    IdOnlyClubRequest,
    DefaultClubRequest,
    DefaultClubRequest,
>;
