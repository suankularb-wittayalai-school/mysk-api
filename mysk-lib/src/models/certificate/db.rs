use crate::{
    helpers::date::get_current_academic_year,
    models::enums::{CertificateType, SubmissionStatus},
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, prelude::FromRow, query};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, GetById)]
#[from_query(
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
    pub async fn is_rsvp_period(conn: &mut PgConnection) -> Result<bool> {
        let res = query!(
            "\
            SELECT EXISTS (\
                SELECT FROM certificate_ceremony_rsvp_periods \
                WHERE now() BETWEEN start_time AND end_time\
            )\
            ",
        )
        .fetch_one(conn)
        .await?;

        Ok(res.exists.unwrap_or(false))
    }

    pub async fn get_rsvp_status(
        conn: &mut PgConnection,
        student_id: Uuid,
    ) -> Result<Option<SubmissionStatus>> {
        let res = query!(
            "\
            SELECT rsvp_status \"rsvp_status: SubmissionStatus\" FROM student_certificates \
            WHERE student_id = $1 AND year = $2 LIMIT 1\
            ",
            student_id,
            get_current_academic_year(None),
        )
        .fetch_one(conn)
        .await?;

        Ok(res.rsvp_status)
    }
}
