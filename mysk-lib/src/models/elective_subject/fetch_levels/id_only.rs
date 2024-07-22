use crate::{
    common::requests::FetchLevel,
    models::{elective_subject::db::DbElectiveSubject, traits::FetchLevelVariant},
    prelude::*,
};
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyElectiveSubject {
    pub id: Uuid,
}

impl_id_only_variant_from!(elective_subject, IdOnlyElectiveSubject, DbElectiveSubject);
