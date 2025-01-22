use crate::{
    common::{
        requests::{FilterConfig, PaginationConfig, QueryParam, SortingConfig, SqlSection},
        response::PaginationType,
    },
    helpers::date::get_current_academic_year,
    models::{
        classroom::ClassroomWClassNo,
        student::request::{queryable::QueryableStudent, sortable::SortableStudent},
        traits::{QueryDb, Queryable as _},
    },
    prelude::*,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, Acquire, FromRow, PgPool, Postgres, QueryBuilder, Row as _};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(
    query = "SELECT id, created_at, student_id, user_id, person_id FROM students",
    count_query = "SELECT COUNT(DISTINCT id) FROM students"
)]
#[get_by_id(table = "students")]
pub struct DbStudent {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub student_id: Option<String>,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
}

impl DbStudent {
    pub async fn get_student_from_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>> {
        let res = query!("SELECT id FROM students WHERE user_id = $1", user_id)
            .fetch_optional(pool)
            .await?;

        Ok(res.map(|r| r.id))
    }

    pub async fn get_student_contacts(pool: &PgPool, student_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "
            SELECT contacts.id FROM contacts
            INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id
            INNER JOIN people ON person_contacts.person_id = people.id
            INNER JOIN students ON people.id = students.person_id
            WHERE students.id = $1
            ",
            student_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.id).collect())
    }

    pub async fn get_student_classroom<'a, A>(
        conn: A,
        student_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<ClassroomWClassNo>>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut conn = conn.acquire().await?;
        let res = query!(
            "
            SELECT classroom_id, class_no FROM classroom_students
            JOIN classrooms ON classrooms.id = classroom_id
            WHERE student_id = $1 AND year = $2
            ",
            student_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(res.map(|res| ClassroomWClassNo {
            id: res.classroom_id,
            class_no: res.class_no,
        }))
    }
}

#[async_trait]
impl QueryDb<QueryableStudent, SortableStudent> for DbStudent {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryableStudent>>,
    ) {
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
                        Some(QueryParam::ArrayString(v)) => query_builder.push_bind(v.clone()),
                        Some(QueryParam::ArrayUuid(v)) => query_builder.push_bind(v.clone()),
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryableStudent>>,
        sort: Option<&SortingConfig<SortableStudent>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>> {
        let mut query = QueryBuilder::new(DbStudent::base_query());
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
                                "DbStudent::query".to_string(),
                            ));
                        }
                    };
                }
            }
        }

        Ok(query.build_query_as::<DbStudent>().fetch_all(pool).await?)
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryableStudent>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType> {
        let mut query = QueryBuilder::new(DbStudent::count_query());
        Self::build_shared_query(&mut query, filter);

        let count = u32::try_from(query.build().fetch_one(pool).await?.get::<i64, _>("count"))
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
