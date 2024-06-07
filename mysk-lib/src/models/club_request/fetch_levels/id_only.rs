use crate::{
    common::requests::FetchLevel,
    models::{club_request::db::DbClubRequest, traits::FetchLevelVariant},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClubRequest {
    pub id: Uuid,
}

impl_id_only_variant_from!(IdOnlyClubRequest, DbClubRequest);
