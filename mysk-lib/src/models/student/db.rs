use crate::{
    common::requests::FilterConfig,
    helpers::date::get_current_academic_year,
    models::{
        classroom::ClassroomWClassNo,
        student::request::{queryable::QueryableStudent, sortable::SortableStudent},
        traits::QueryDb,
    },
    prelude::*,
    query::Queryable as _,
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, Postgres, QueryBuilder, query};
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
    pub async fn get_student_from_user_id(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Option<Uuid>> {
        let res = query!("SELECT id FROM students WHERE user_id = $1", user_id)
            .fetch_optional(conn)
            .await?;

        Ok(res.map(|r| r.id))
    }

    pub async fn get_student_contacts(
        conn: &mut PgConnection,
        student_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT contacts.id FROM contacts \
            INNER JOIN person_contacts ON contacts.id = person_contacts.contact_id \
            INNER JOIN people ON person_contacts.person_id = people.id \
            INNER JOIN students ON people.id = students.person_id \
            WHERE students.id = $1\
            ",
            student_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|r| r.id).collect())
    }

    pub async fn get_student_classroom(
        conn: &mut PgConnection,
        student_id: Uuid,
        academic_year: Option<i64>,
    ) -> Result<Option<ClassroomWClassNo>> {
        let res = query!(
            "\
            SELECT classroom_id, class_no FROM classroom_students \
            JOIN classrooms ON classrooms.id = classroom_id \
            WHERE student_id = $1 AND year = $2\
            ",
            student_id,
            match academic_year {
                Some(year) => year,
                None => get_current_academic_year(None),
            },
        )
        .fetch_optional(conn)
        .await?;

        Ok(res.map(|res| ClassroomWClassNo {
            id: res.classroom_id,
            class_no: res.class_no,
        }))
    }
}

impl QueryDb<QueryableStudent, SortableStudent> for DbStudent {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableStudent>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = filter.data {
                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
