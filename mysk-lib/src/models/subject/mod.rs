use crate::models::{
    subject::{
        db::DbSubject,
        fetch_levels::{
            compact::CompactSubject, default::DefaultSubject, detailed::DetailedSubject,
            id_only::IdOnlySubject,
        },
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;

pub type Subject =
    Model<DbSubject, IdOnlySubject, CompactSubject, DefaultSubject, DetailedSubject>;
