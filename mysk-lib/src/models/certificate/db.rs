use crate::{
    models::enums::{CertificateType, SubmissionStatus},
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query, Acquire, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM student_certificates",
    count_query = "SELECT COUNT(*) FROM student_certificates"
)]
pub struct DbCertificate {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub student_id: Uuid,
    pub certificate_type: CertificateType,
    pub certificate_detail: String,
    pub year: i64,
    pub receiving_order_number: Option<i64>,
    pub seat_code: Option<String>,
    pub rsvp_status: Option<SubmissionStatus>,
}

impl DbCertificate {
    pub async fn is_rsvp_period<'a, A>(conn: A) -> Result<bool>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let res = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM certificate_ceremony_rsvp_periods \
                WHERE now() BETWEEN start_time AND end_time\
            )\
            ",
        )
        .fetch_one(&mut *(conn.acquire().await?))
        .await?;

        Ok(res.exists.unwrap_or(false))
    }
}
