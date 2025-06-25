use crate::models::{
    elective_trade_offer::{
        db::DbElectiveTradeOffer,
        fetch_levels::{default::DefaultElectiveTradeOffer, id_only::IdOnlyElectiveTradeOffer},
        request::{queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer},
    },
    top_level_variant::TopLevelVariant,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveTradeOffer = TopLevelVariant<
    DbElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    DefaultElectiveTradeOffer,
    DefaultElectiveTradeOffer,
    QueryableElectiveTradeOffer,
    SortableElectiveTradeOffer,
>;
