use crate::models::certificate::db::DbCertificate;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyCertificate {
    pub id: Uuid,
}

impl_id_only_variant_from!(certificate, IdOnlyCertificate, DbCertificate);
