use crate::models::{
    club::{
        db::DbClub,
        fetch_levels::{
            compact::CompactClub, default::DefaultClub, detailed::DetailedClub, id_only::IdOnlyClub,
        },
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type Club = Model<DbClub, IdOnlyClub, CompactClub, DefaultClub, DetailedClub>;
