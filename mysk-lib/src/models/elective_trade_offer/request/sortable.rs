use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableElectiveTradeOffer {
    Id,
    SenderId,
    ReceiverId,
    Status,
    CreatedAt,
}

impl Default for SortableElectiveTradeOffer {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableElectiveTradeOffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortableElectiveTradeOffer::Id => write!(f, "id"),
            SortableElectiveTradeOffer::SenderId => write!(f, "sender_id"),
            SortableElectiveTradeOffer::ReceiverId => write!(f, "receiver_id"),
            SortableElectiveTradeOffer::Status => write!(f, "status"),
            SortableElectiveTradeOffer::CreatedAt => write!(f, "created_at"),
        }
    }
}
