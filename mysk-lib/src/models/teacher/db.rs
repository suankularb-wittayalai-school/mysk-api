use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    error::Error,
    helpers::date::get_current_academic_year,
    models::{
        teacher::request::{queryable::QueryableTeacher, sortable::SortableTeacher},
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(
    query = "
    SELECT id, created_at, teacher_id, subject_group_id, user_id, person_id FROM teachers",
    count_query = "SELECT COUNT(distinct id) FROM teachers"
)]
#[get_by_id(table = "teachers")]
pub struct DbTeacher {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub teacher_id: Option<String>,
    pub subject_group_id: i64,
    pub person_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

impl DbTeacher {
    pub async fn get_teacher_contacts(pool: &PgPool, teacher_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "
            SELECT contacts.id FROM contacts
            INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id
            INNER JOIN people ON person_contacts.person_id = people.id
            INNER JOIN teachers ON people.id = teachers.person_id
            WHERE teachers.id = $1
            ",
            teacher_id,
        )
        .fetch_all(pool)
        .await;

        match res {
            Ok(res) => Ok(res.iter().map(|r| r.id).collect()),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbTeacher::get_teacher_contacts".to_string(),
            )),
        }
    }

    pub async fn get_teacher_advisor_at(
        pool: &PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<Uuid>> {
        let res = query!(
            "
            SELECT classroom_id FROM classroom_advisors
            INNER JOIN classrooms ON classrooms.id = classroom_id
            WHERE teacher_id = $1 AND classrooms.year = $2
            ",
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_optional(pool)
        .await;

        match res {
            Ok(res) => Ok(res.map(|r| r.classroom_id)),
            Err(e) => Err(Error::InternalSeverError(
                e.to_string(),
                "DbTeacher::get_teacher_advisor_at".to_string(),
            )),
        }
    }

    pub async fn get_subject_in_charge(
        pool: &PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let as_teacher = query!(
            "SELECT subject_id FROM subject_teachers WHERE teacher_id = $1 AND year = $2",
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_all(pool)
        .await;
        let as_co_teacher = query!(
            "SELECT subject_id FROM subject_co_teachers WHERE teacher_id = $1 AND year = $2",
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_all(pool)
        .await;

        let as_teacher = match as_teacher {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InternalSeverError(
                    e.to_string(),
                    "DbTeacher::get_subject_in_charge".to_string(),
                ));
            }
        };
        let as_co_teacher = match as_co_teacher {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InternalSeverError(
                    e.to_string(),
                    "DbTeacher::get_subject_in_charge".to_string(),
                ));
            }
        };

        let mut result = Vec::with_capacity(as_teacher.len() + as_co_teacher.len());
        for record in as_teacher {
            result.push(record.subject_id);
        }
        for record in as_co_teacher {
            result.push(record.subject_id);
        }

        Ok(result)
    }
}

#[async_trait]
impl QueryDb<QueryableTeacher, SortableTeacher> for DbTeacher {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableTeacher>>,
    ) where
        Self: Sized,
    {
        let mut where_sections = Vec::<SqlSection>::new();

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
                        Some(QueryParam::Int(v)) => query_builder.push_bind(v.clone()),
                        Some(QueryParam::ArrayUuid(v)) => query_builder.push_bind(v.clone()),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableTeacher>>,
        sort: Option<&SortingConfig<SortableTeacher>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: BaseQuery + Sized,
    {
        let mut query = QueryBuilder::new(DbTeacher::base_query());
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
                                "DbTeacher::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        query
            .build_query_as::<DbTeacher>()
            .fetch_all(pool)
            .await
            .map_err(|e| Error::InternalSeverError(e.to_string(), "DbTeacher::query".to_string()))
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableTeacher>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType>
    where
        Self: Sized,
    {
        let mut query = QueryBuilder::new(DbTeacher::count_query());
        Self::build_shared_query(&mut query, filter);

        let count = u32::try_from(
            query
                .build()
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    Error::InternalSeverError(
                        e.to_string(),
                        "DbTeacher::response_pagination".to_string(),
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
