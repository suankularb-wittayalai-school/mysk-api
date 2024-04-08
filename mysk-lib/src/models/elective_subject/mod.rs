use self::{
    db::DbElectiveSubject,
    fetch_levels::{
        compact::CompactElectiveSubject, default::DefaultElectiveSubject,
        detailed::DetailedElectiveSubject, id_only::IdOnlyElectiveSubject,
    },
};

use super::common::top_level_variant::TopLevelVariant;

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveSubject = TopLevelVariant<
    DbElectiveSubject,
    IdOnlyElectiveSubject,
    CompactElectiveSubject,
    DefaultElectiveSubject,
    DetailedElectiveSubject,
>;
