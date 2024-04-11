use self::{
    db::DbElectiveTradeOffer,
    fetch_levels::{default::DefaultElectiveTradeOffer, id_only::IdOnlyElectiveTradeOffer},
};
use super::common::top_level_variant::TopLevelVariant;

pub mod db;
pub mod fetch_levels;

pub type ElectiveTradeOffer = TopLevelVariant<
    DbElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    DefaultElectiveTradeOffer,
    DefaultElectiveTradeOffer,
>;
