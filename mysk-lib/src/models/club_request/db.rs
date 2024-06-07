use crate::{
    common::{
        requests::{PaginationConfig, QueryParam, SqlSection},
        response::PaginationType,
    },
    error::Error,
    models::{
        club_request::request::{queryable::QueryableClubRequest, sortable::SortableClubRequest},
        enums::SubmissionStatus,
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Row as _};
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

impl QueryDb<QueryableClubRequest, SortableClubRequest> for DbClubRequest {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&crate::common::requests::FilterConfig<QueryableClubRequest>>,
    ) where
        Self: Sized,
    {
        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(data) = &filter.data {
                let mut data_sections = data.to_query_string();
                where_sections.append(&mut data_sections);
            }
        }

        for (i, section) in where_sections.iter().enumerate() {
            query_builder.push(if i == 0 { " WHERE " } else { " AND " });
            for (j, sql) in section.sql.iter().enumerate() {
                query_builder.push(sql);
                if j < section.params.len() {
                    match section.params.get(j) {
                        Some(QueryParam::ArrayUuid(v)) => query_builder.push_bind(v.clone()),
                        Some(QueryParam::Int(v)) => query_builder.push_bind(*v),
                        Some(QueryParam::SubmissionStatus(v)) => query_builder.push_bind(*v),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&crate::common::requests::FilterConfig<QueryableClubRequest>>,
        sort: Option<&crate::common::requests::SortingConfig<SortableClubRequest>>,
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
        filter: Option<&crate::common::requests::FilterConfig<QueryableClubRequest>>,
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
