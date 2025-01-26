use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    models::{
        online_teaching_reports::requests::{
            queryable::QueryableOnlineTeachingReports, sortable::SortableOnlineTeachingReports,
        },
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM online_teaching_reports",
    count_query = "SELECT COUNT(*) FROM online_teaching_reports"
)]
pub struct DbOnlineTeachingReports {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub subject_id: Uuid,
    pub teacher_id: Uuid,
    pub classroom_id: Uuid,
    pub date: NaiveDate,
    pub teaching_methods: Vec<String>,
    pub teaching_topic: String,
    pub suggestions: Option<String>,
    pub absent_student_no: Option<Vec<i64>>,
    pub start_time: i64,
    pub duration: i64,
    pub has_image: bool,
}

#[async_trait]
impl QueryDb<QueryableOnlineTeachingReports, SortableOnlineTeachingReports>
    for DbOnlineTeachingReports
{
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableOnlineTeachingReports>>,
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
                        Some(QueryParam::Uuid(v)) => query_builder.push_bind(*v),
                        Some(QueryParam::ArrayUuid(v)) => query_builder.push_bind(v.clone()),
                        Some(QueryParam::ArrayNaiveDate(v)) => query_builder.push_bind(v.clone()),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableOnlineTeachingReports>>,
        sort: Option<&SortingConfig<SortableOnlineTeachingReports>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::new(DbOnlineTeachingReports::base_query());
        Self::build_shared_query(&mut query, filter);

        if let Some(sorting) = sort {
            query.push(sorting.to_order_by_clause());
        }

        if let Some(pagination) = pagination {
            let limit_section = pagination.to_limit_clause()?;
            query.push(" ");
            for (i, sql) in limit_section.sql.iter().enumerate() {
                query.push(sql);
                if i < limit_section.params.len() {
                    match limit_section.params.get(i) {
                        Some(&QueryParam::Int(v)) => query.push_bind(v),
                        _ => {
                            return Err(Error::InternalSeverError(
                                "Invalid pagination params".to_string(),
                                "DbOnlineTeachingReport::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        query
            .build_query_as::<DbOnlineTeachingReports>()
            .fetch_all(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(
                    e.to_string(),
                    "DbOnlineTeachingReports::query".to_string(),
                )
            })
    }

    async fn response_pagination(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableOnlineTeachingReports>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        let mut query = QueryBuilder::new(DbOnlineTeachingReports::count_query());
        Self::build_shared_query(&mut query, filter);

        let count = u32::try_from(
            query
                .build()
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    Error::InternalSeverError(
                        e.to_string(),
                        "DbOnlineTeachingReports::response_pagination".to_string(),
                    )
                })?
                .get::<i64, _>("count"),
        )
        .expect("Irrecoverable error, i64 is out of bounds for u32");

        Ok(PaginationType::new(
            pagination.unwrap_or(&PaginationConfig::default()).p,
            pagination
                .unwrap_or(&PaginationConfig::default())
                .size
                .unwrap_or(50),
            count,
        ))
    }
}
