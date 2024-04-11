use self::{
    db::DbElectiveSubject,
    fetch_levels::{
        compact::CompactElectiveSubject, default::DefaultElectiveSubject,
        detailed::DetailedElectiveSubject, id_only::IdOnlyElectiveSubject,
    },
    request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
};
use crate::models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};

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

impl TopLevelQuery<DbElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>
    for ElectiveSubject
{
}
