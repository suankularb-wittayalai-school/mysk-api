use crate::models::{
    cheer_practice_period::{
        db::DbCheerPracticePeriod,
        fetch_levels::{
            default::DefaultCheerPracticePeriod, detailed::DetailedCheerPracticePeriod,
            id_only::IdOnlyCheerPracticePeriod,
        },
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type CheerPracticePeriod = Model<
    DbCheerPracticePeriod,
    IdOnlyCheerPracticePeriod,
    DefaultCheerPracticePeriod,
    DefaultCheerPracticePeriod,
    DetailedCheerPracticePeriod,
>;
