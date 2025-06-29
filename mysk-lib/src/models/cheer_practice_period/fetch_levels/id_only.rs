use crate::models::cheer_practice_period::db::DbCheerPracticePeriod;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyCheerPracticePeriod {
    pub id: Uuid,
}

impl_id_only_variant_from!(
    cheer_practice_period,
    IdOnlyCheerPracticePeriod,
    DbCheerPracticePeriod,
);
