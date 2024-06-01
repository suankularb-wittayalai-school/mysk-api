use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    error::Error,
    helpers::date::get_current_academic_year,
    models::{
        club::request::{queryable::QueryableClub, sortable::SortableClub},
        enums::{ActivityDayHouse, SubmissionStatus},
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
    query = "SELECT * FROM clubs_with_detail_view",
    count_query = "SELECT COUNT(*) FROM clubs"
)]
pub struct DbClub {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub organization_id: Uuid,
    pub accent_color: Option<String>,
    pub background_color: Option<String>,
    pub description_en: Option<String>,
    pub description_th: Option<String>,
    pub logo_url: Option<String>,
    pub member_count: i64,
    pub name_en: Option<String>,
    pub name_th: String,
    pub staff_count: i64,
}

impl DbClub {
    pub async fn get_club_contacts(pool: &PgPool, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT contact_id FROM club_contacts WHERE club_id = $1",
            club_id,
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.contact_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClub::get_club_contacts".to_string(),
            )),
        }
    }

    pub async fn get_club_members(pool: &PgPool, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "
            SELECT student_id FROM club_members
            WHERE club_id = $1 AND year = $2 AND membership_status = $3
            ",
            club_id,
            get_current_academic_year(None),
            SubmissionStatus::Approved as SubmissionStatus,
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.student_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClub::get_club_members".to_string(),
            )),
        }
    }

    pub async fn get_club_staffs(pool: &PgPool, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT student_id FROM club_staffs WHERE club_id = $1 AND year = $2",
            club_id,
            get_current_academic_year(None),
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.student_id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbClub::get_club_staffs".to_string(),
            )),
        }
    }
}

impl QueryDb<QueryableClub, SortableClub> for DbClub {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableClub>>,
    ) where
        Self: Sized,
    {
        let mut where_sections: Vec<SqlSection> = Vec::new();

        if let Some(filter) = filter {
            if let Some(q) = &filter.q {
                // (organizations.name_th ILIKE '%q%' OR organizations.name_en ILIKE '%q%' OR
                // organizations.description_th ILIKE '%q%' OR organizations.description_en ILIKE
                // '%q%')
                where_sections.push(SqlSection {
                    sql: vec![
                        "(organizations.name_th ILIKE concat('%', ".to_string(),
                        ", '%') OR organizations.name_en ILIKE concat('%', ".to_string(),
                        ", '%') OR organizations.description_th ILIKE concat('%', ".to_string(),
                        ", '%') OR organizations.description_en ILIKE concat('%', ".to_string(),
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
            query_builder.push(if i == 0 { " WHERE " } else { " AND " });
            for (j, sql) in section.sql.iter().enumerate() {
                query_builder.push(sql);
                if j < section.params.len() {
                    match section.params.get(j) {
                        Some(QueryParam::Uuid(v)) => query_builder.push_bind(*v),
                        Some(QueryParam::String(v)) => query_builder.push_bind(v.clone()),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableClub>>,
        sort: Option<&SortingConfig<SortableClub>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: BaseQuery + Sized,
    {
        let mut query = QueryBuilder::new(DbClub::base_query());
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
                                "DbClub::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        query
            .build_query_as::<DbClub>()
            .fetch_all(pool)
            .await
            .map_err(|e| Error::InternalSeverError(e.to_string(), "DbClub::query".to_string()))
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableClub>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::new(DbClub::count_query());
        Self::build_shared_query(&mut query, filter);

        let count = u32::try_from(
            query
                .build()
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    Error::InternalSeverError(
                        e.to_string(),
                        "DbClub::response_pagination".to_string(),
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
