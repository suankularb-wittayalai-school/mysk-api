use crate::models::club_request::db::DbClubRequest;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClubRequest {
    pub id: Uuid,
}

impl_id_only_variant_from!(club_request, IdOnlyClubRequest, DbClubRequest);
