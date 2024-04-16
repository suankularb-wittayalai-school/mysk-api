use self::{
    db::DbElectiveTradeOffer,
    fetch_levels::{default::DefaultElectiveTradeOffer, id_only::IdOnlyElectiveTradeOffer},
    request::{queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer},
};
use crate::models::top_level_variant::TopLevelVariant;

use super::traits::TopLevelQuery;

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveTradeOffer = TopLevelVariant<
    DbElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    DefaultElectiveTradeOffer,
    DefaultElectiveTradeOffer,
>;

impl TopLevelQuery<DbElectiveTradeOffer, QueryableElectiveTradeOffer, SortableElectiveTradeOffer>
    for ElectiveTradeOffer
{
}
