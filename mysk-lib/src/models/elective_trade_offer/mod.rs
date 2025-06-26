use crate::models::{
    elective_trade_offer::{
        db::DbElectiveTradeOffer,
        fetch_levels::{default::DefaultElectiveTradeOffer, id_only::IdOnlyElectiveTradeOffer},
    },
    model::Model,
};

pub mod db;
pub mod fetch_levels;
pub mod request;

pub type ElectiveTradeOffer = Model<
    DbElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    IdOnlyElectiveTradeOffer,
    DefaultElectiveTradeOffer,
    DefaultElectiveTradeOffer,
>;
