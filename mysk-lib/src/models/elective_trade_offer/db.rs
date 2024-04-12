use super::request::{
    queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer,
};
use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    models::{
        enums::submission_status::SubmissionStatus,
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(query = "SELECT * FROM elective_subject_trade_offers")]
pub struct DbElectiveTradeOffer {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub sender_elective_subject_id: Uuid,
    pub receiver_elective_subject_id: Uuid,
    pub status: SubmissionStatus,
}

impl QueryDb<QueryableElectiveTradeOffer, SortableElectiveTradeOffer> for DbElectiveTradeOffer {
    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableElectiveTradeOffer>>,
        sort: Option<&SortingConfig<SortableElectiveTradeOffer>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::<'_, Postgres>::new(DbElectiveTradeOffer::base_query());
        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(data) = &filter.data {
                let mut data_sections = data.to_query_string();
                where_sections.append(&mut data_sections);
            }
        }

        for (i, section) in where_sections.iter().enumerate() {
            query.push(if i == 0 { " WHERE " } else { " AND " });
            for (j, sql) in section.sql.iter().enumerate() {
                query.push(sql);
                if j < section.params.len() {
                    match &section.params[j] {
                        QueryParam::ArrayUuid(v) => query.push_bind(v),
                        QueryParam::SubmissionStatus(v) => query.push_bind(v),
                        _ => unreachable!(),
                    };
                }
            }
        }

        if let Some(sorting) = sort {
            query.push(sorting.to_order_by_clause());
        }

        if let Some(pagination) = pagination {
            let limit_section = pagination.to_limit_clause();
            query.push(" ");
            for (i, sql) in limit_section.sql.iter().enumerate() {
                query.push(sql);
                if i < limit_section.params.len() {
                    match limit_section.params[i] {
                        QueryParam::Int(v) => query.push_bind(v),
                        _ => {
                            return Err(Error::InternalSeverError(
                                "Invalid pagination params".to_string(),
                                "DbElectiveTradeOffer::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        query
            .build_query_as::<DbElectiveTradeOffer>()
            .fetch_all(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(e.to_string(), "DbElectiveTradeOffer::query".to_string())
            })
    }

    async fn response_pagination(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableElectiveTradeOffer>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        let mut query =
            QueryBuilder::<'_, sqlx::Postgres>::new(DbElectiveTradeOffer::count_query());

        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(q) = &filter.q {
                // (name_th ILIKE '%q%' OR name_en ILIKE '%q%' OR code_th ILIKE '%q%' OR code_en ILIKE '%q%')
                where_sections.push(SqlSection {
                    sql: vec![
                        "(name_th ILIKE concat('%', ".to_string(),
                        ", '%') OR name_en ILIKE concat('%', ".to_string(),
                        ", '%') OR code_th ILIKE concat('%', ".to_string(),
                        ", '%') OR code_en ILIKE concat('%', ".to_string(),
                        ", '%'))".to_string(),
                    ],
                    params: vec![
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                        QueryParam::String(q.to_string()),
                    ],
                });
            }
            if let Some(data) = &filter.data {
                let mut data_sections = data.to_query_string();
                where_sections.append(&mut data_sections);
            }
        }

        for (i, section) in where_sections.iter().enumerate() {
            query.push(if i == 0 { " WHERE " } else { " AND " });
            for (j, sql) in section.sql.iter().enumerate() {
                query.push(sql);
                if j < section.params.len() {
                    match &section.params[j] {
                        QueryParam::ArrayUuid(v) => query.push_bind(v),
                        QueryParam::SubmissionStatus(v) => query.push_bind(v),
                        _ => unreachable!(),
                    };
                }
            }
        }

        let count = query
            .build()
            .fetch_one(pool)
            .await
            .map_err(|e| {
                Error::InternalSeverError(
                    e.to_string(),
                    "DbElectiveTradeOffer::response_pagination".to_string(),
                )
            })?
            .get::<i64, _>("count") as u32;

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
