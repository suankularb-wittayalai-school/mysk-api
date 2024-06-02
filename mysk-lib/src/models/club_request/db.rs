use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    error::Error,
    helpers::date::get_current_academic_year,
    models::{
        enums::SubmissionStatus,
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(
    query = "SELECT student_id FROM club_members",
    count_query = "SELECT COUNT(student_id) FROM club_members"
)]
pub struct DbClubRequest {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub club_id: Uuid,
    pub year: Option<i64>,
    pub membership_status: SubmissionStatus,
    pub student_id: Uuid,
}

impl DbClubRequest {
    pub async fn get_club_requests(pool: &PgPool, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT student_id FROM club_members WHERE club_id = $1",
            club_id,
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.student_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClubRequest::get_club_requests".to_string(),
            )),
        }
    }
}

impl QueryDb<QueryablePlaceholder, SortablePlaceholder> for DbClubRequest {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&crate::common::requests::FilterConfig<QueryablePlaceholder>>,
    ) where
        Self: Sized,
    {
        todo!()
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&crate::common::requests::FilterConfig<QueryablePlaceholder>>,
        sort: Option<&crate::common::requests::SortingConfig<SortablePlaceholder>>,
        pagination: Option<&crate::common::requests::PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: BaseQuery + Sized,
    {
        todo!()
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&crate::common::requests::FilterConfig<QueryablePlaceholder>>,
        pagination: Option<&crate::common::requests::PaginationConfig>,
    ) -> Result<crate::common::response::PaginationType>
    where
        Self: Sized,
    {
        todo!()
    }
}
