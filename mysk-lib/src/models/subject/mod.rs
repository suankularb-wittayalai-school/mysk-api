use crate::models::{
    subject::{
        db::DbSubject,
        fetch_levels::{
            compact::CompactSubject, default::DefaultSubject, detailed::DetailedSubject,
            id_only::IdOnlySubject,
        },
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;

pub type Subject =
    TopLevelVariant<DbSubject, IdOnlySubject, CompactSubject, DefaultSubject, DetailedSubject>;
