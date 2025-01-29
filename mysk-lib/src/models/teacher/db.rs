use crate::{
    common::requests::FilterConfig,
    helpers::date::get_current_academic_year,
    models::{
        teacher::request::{queryable::QueryableTeacher, sortable::SortableTeacher},
        traits::QueryDb,
    },
    prelude::*,
    query::Queryable as _,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool, Postgres, QueryBuilder};
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
    pub async fn get_teacher_from_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>> {
        let res = query!("SELECT id FROM teachers WHERE user_id = $1", user_id)
            .fetch_optional(pool)
            .await?;

        Ok(res.map(|r| r.id))
    }

    pub async fn get_teacher_contacts(pool: &PgPool, teacher_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT contacts.id FROM contacts \
            INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id \
            INNER JOIN people ON person_contacts.person_id = people.id \
            INNER JOIN teachers ON people.id = teachers.person_id \
            WHERE teachers.id = $1\
            ",
            teacher_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(res.into_iter().map(|r| r.id).collect())
    }

    pub async fn get_teacher_advisor_at(
        pool: &PgPool,
        teacher_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<Uuid>> {
        let res = query!(
            "\
            SELECT classroom_id FROM classroom_advisors \
            INNER JOIN classrooms ON classrooms.id = classroom_id \
            WHERE teacher_id = $1 AND classrooms.year = $2\
            ",
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_optional(pool)
        .await?;

        Ok(res.map(|r| r.classroom_id))
    }

    pub async fn get_teacher_subject_group(pool: &PgPool, teacher_id: Uuid) -> Result<Option<i64>> {
        let res = query!(
            "
            SELECT subject_group_id FROM teachers WHERE id = $1
            ",
            teacher_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(res.map(|r| r.subject_group_id))
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
        .await?;
        let as_co_teacher = query!(
            "SELECT subject_id FROM subject_co_teachers WHERE teacher_id = $1 AND year = $2",
            teacher_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::with_capacity(as_teacher.len() + as_co_teacher.len());
        as_teacher
            .into_iter()
            .for_each(|record| result.push(record.subject_id));
        as_co_teacher
            .into_iter()
            .for_each(|record| result.push(record.subject_id));

        Ok(result)
    }
}

#[async_trait]
impl QueryDb<QueryableTeacher, SortableTeacher> for DbTeacher {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableTeacher>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
