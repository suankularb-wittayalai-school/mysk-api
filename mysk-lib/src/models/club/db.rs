use crate::{
    common::requests::FilterConfig,
    helpers::date::get_current_academic_year,
    models::{
        club::request::{queryable::QueryableClub, sortable::SortableClub},
        enums::SubmissionStatus,
        traits::QueryDb,
    },
    prelude::*,
    query::{QueryParam, Queryable as _, SqlWhereClause},
};
use chrono::{DateTime, Utc};
use mysk_lib_macros::GetById;
use serde::Deserialize;
use sqlx::{FromRow, PgConnection, Postgres, QueryBuilder, query};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, FromRow, GetById)]
#[from_query(
    query = "SELECT * FROM clubs_with_detail_view",
    count_query = "SELECT COUNT(*) FROM clubs_with_detail_view"
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
    pub async fn get_club_contacts(conn: &mut PgConnection, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT contact_id FROM club_contacts WHERE club_id = $1",
            club_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|r| r.contact_id).collect())
    }

    pub async fn get_club_members(conn: &mut PgConnection, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "\
            SELECT student_id FROM club_members \
            WHERE club_id = $1 AND year = $2 AND membership_status = $3\
            ",
            club_id,
            get_current_academic_year(None),
            SubmissionStatus::Approved as SubmissionStatus,
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|r| r.student_id).collect())
    }

    pub async fn get_club_staffs(conn: &mut PgConnection, club_id: Uuid) -> Result<Vec<Uuid>> {
        let res = query!(
            "SELECT student_id FROM club_staffs WHERE club_id = $1 AND year = $2",
            club_id,
            get_current_academic_year(None),
        )
        .fetch_all(conn)
        .await?;

        Ok(res.into_iter().map(|r| r.student_id).collect())
    }
}

impl QueryDb<QueryableClub, SortableClub> for DbClub {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableClub>>,
    ) {
        if let Some(filter) = filter {
            let query_is_none = filter.q.is_none();

            if let Some(query) = filter.q {
                let mut wc = SqlWhereClause::new();
                wc.push_sql("name_th ILIKE ('%' || ")
                    .push_param(QueryParam::String(query))
                    .push_sql(" || '%') OR name_en ILIKE ('%' || ")
                    .push_prev_param()
                    .push_sql(" || '%') OR description_th ILIKE ('%' || ")
                    .push_prev_param()
                    .push_sql(" || '%') OR description_en ILIKE ('%' || ")
                    .push_prev_param()
                    .push_sql(" || '%')");

                wc.append_into_query_builder(query_builder);
            }

            if let Some(data) = filter.data {
                if query_is_none {
                    query_builder.push(" WHERE ");
                }

                data.to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
