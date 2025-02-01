use crate::models::elective_trade_offer::db::DbElectiveTradeOffer;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyElectiveTradeOffer {
    pub id: Uuid,
}

impl_id_only_variant_from!(
    elective_trade_offer,
    IdOnlyElectiveTradeOffer,
    DbElectiveTradeOffer,
);
