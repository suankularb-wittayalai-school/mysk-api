use crate::models::{
    elective_subject::{
        db::DbElectiveSubject,
        fetch_levels::{
            compact::CompactElectiveSubject, default::DefaultElectiveSubject,
            detailed::DetailedElectiveSubject, id_only::IdOnlyElectiveSubject,
        },
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveSubject = Model<
    DbElectiveSubject,
    IdOnlyElectiveSubject,
    CompactElectiveSubject,
    DefaultElectiveSubject,
    DetailedElectiveSubject,
>;
