use crate::{
    common::{requests::FetchLevel, string::FlexibleMultiLangString},
    models::{certificate::db::DbCertificate, traits::FetchVariant},
    permissions::{ActionType, Authorizable as _, Authorizer},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultCertificate {
    pub id: Uuid,
    pub student_id: Uuid,
    pub certificate_type: String,
    pub certificate_detail: Option<FlexibleMultiLangString>,
    pub year: i64,
    pub receiving_order_number: Option<i64>,
    pub seat_code: Option<String>,
    pub rsvp_status: Option<String>,
}

impl FetchVariant for DefaultCertificate {
    type Relation = DbCertificate;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        _descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        authorizer
            .authorize_certificate(
                &relation,
                &mut *(pool.acquire().await?),
                ActionType::ReadDefault,
            )
            .await?;

        Ok(Self {
            id: relation.id,
            student_id: relation.student_id,
            certificate_type: relation.certificate_type.to_string(),
            certificate_detail: Some(FlexibleMultiLangString {
                th: Some(relation.certificate_detail),
                en: None,
            }),
            year: relation.year,
            receiving_order_number: relation.receiving_order_number,
            seat_code: relation.seat_code,
            rsvp_status: relation.rsvp_status.map(|status| status.to_string()),
        })
    }
}
