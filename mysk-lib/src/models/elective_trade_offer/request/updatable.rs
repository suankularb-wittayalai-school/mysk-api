use crate::models::enums::submission_status::SubmissionStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatableElectiveOffer {
    pub status: SubmissionStatus,
}
