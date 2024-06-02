use crate::{
    common::{
        requests::{
            FilterConfig, PaginationConfig, QueryParam, QueryablePlaceholder, SortablePlaceholder,
            SortingConfig, SqlSection,
        },
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
        let mut query = QueryBuilder::new(DbClubRequest::base_query());
        Self::build_shared_query(&mut query, filter);

        if let Some(sorting) = sort {
            query.push(sorting.to_order_by_clause());
        }

        if let Some(pagination) = pagination {
            let limit_section = pagination.to_limit_clause();
            query.push(" ");
            for (i, sql) in limit_section.sql.iter().enumerate() {
                query.push(sql);
                if i < limit_section.params.len() {
                    match limit_section.params.get(i) {
                        Some(&QueryParam::Int(v)) => query.push_bind(v),
                        _ => {
                            return Err(Error::InternalSeverError(
                                "Invalid pagination params".to_string(),
                                "DbClubRequest::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        query
            .build_query_as::<DbClubRequest>()
            .fetch_all(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(e.to_string(), "DbClubRequest::query".to_string())
            })
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&crate::common::requests::FilterConfig<QueryablePlaceholder>>,
        pagination: Option<&crate::common::requests::PaginationConfig>,
    ) -> Result<crate::common::response::PaginationType>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::new(DbClubRequest::count_query());
        Self::build_shared_query(&mut query, filter);

        let count = u32::try_from(
            query
                .build()
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    Error::InternalSeverError(
                        e.to_string(),
                        "DbClubRequest::response_pagination".to_string(),
                    )
                })?
                .get::<i64, _>("count"),
        )
        .unwrap();

        match pagination {
            Some(pagination) => Ok(PaginationType::new(
                pagination.p,
                pagination.size.unwrap(),
                count,
            )),
            None => Ok(PaginationType::new(
                PaginationConfig::default().p,
                PaginationConfig::default().size.unwrap(),
                count,
            )),
        }
    }
}
