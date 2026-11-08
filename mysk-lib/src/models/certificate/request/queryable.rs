use crate::{
    models::certificate::db::DbCertificate,
    query::{QueryParam, Queryable, SqlWhereClause},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableCertificate {
    pub ids: Option<Vec<Uuid>>,
    pub student_ids: Option<Vec<Uuid>>,
    pub certificate_type: Option<String>,
    pub certificate_detail: Option<String>,
    pub year: Option<i64>,
    pub receiving_order_number: Option<i64>,
    pub seat_code: Option<String>,
    pub rsvp_status: Option<String>,
}

impl Queryable for QueryableCertificate {
    type Relation = DbCertificate;

    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new_empty();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.student_ids, |mut f, student_ids| {
            f.push_sql("student_id = ANY(")
                .push_param(QueryParam::ArrayUuid(student_ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.certificate_type, |mut f, certificate_type| {
            f.push_sql("certificate_type = ")
                .push_param(QueryParam::String(certificate_type));

            f
        })
        .push_if_some(self.certificate_detail, |mut f, detail| {
            f.push_sql("certificate_detail ILIKE ('%' || ")
                .push_param(QueryParam::String(detail))
                .push_sql(" || '%')");

            f
        })
        .push_if_some(self.year, |mut f, year| {
            f.push_sql("year = ").push_param(QueryParam::Int(year));

            f
        })
        .push_if_some(
            self.receiving_order_number,
            |mut f, receiving_order_number| {
                f.push_sql("receiving_order_number = ")
                    .push_param(QueryParam::Int(receiving_order_number));

                f
            },
        )
        .push_if_some(self.seat_code, |mut f, seat_code| {
            f.push_sql("seat_code = ")
                .push_param(QueryParam::String(seat_code));

            f
        })
        .push_if_some(self.rsvp_status, |mut f, rsvp_status| {
            f.push_sql("rsvp_status = ")
                .push_param(QueryParam::String(rsvp_status));

            f
        });

        wc
    }
}
