use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatableElectiveTradeOffer {
    pub receiver_id: Uuid,
}
