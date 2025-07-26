use crate::models::{
    model::Model,
    subject::{
        db::DbSubject,
        fetch_levels::{
            compact::CompactSubject, default::DefaultSubject, detailed::DetailedSubject,
            id_only::IdOnlySubject,
        },
    },
};

pub mod db;
pub mod fetch_levels;

pub type Subject = Model<DbSubject, IdOnlySubject, CompactSubject, DefaultSubject, DetailedSubject>;
