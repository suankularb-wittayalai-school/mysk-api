use crate::{
    common::requests::FilterConfig,
    models::{
        club_request::request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
        enums::SubmissionStatus,
        traits::QueryDb,
    },
    query::Queryable as _,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT id, created_at, club_id, year, membership_status, student_id FROM club_members",
    count_query = "SELECT COUNT(*) FROM club_members"
)]
pub struct DbClubRequest {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub club_id: Uuid,
    pub year: Option<i64>,
    pub membership_status: SubmissionStatus,
    pub student_id: Uuid,
}

impl QueryDb<QueryableClubRequest, SortableClubRequest> for DbClubRequest {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableClubRequest>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
