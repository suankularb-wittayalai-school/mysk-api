use crate::{
    common::{
        requests::{
            FilterConfig, PaginationConfig, QueryablePlaceholder,
            SortingConfig,
        },
        response::PaginationType,
    },
    error::Error,
    helpers::date::get_current_academic_year,
    models::{
        club::request::sortable::SortableClub,
        enums::{ActivityDayHouse, SubmissionStatus},
        traits::QueryDb,
    },
    prelude::*,
};
use chrono::{DateTime, Utc};
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::Deserialize;
use sqlx::{query, FromRow, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById)]
#[base_query(query = "SELECT * FROM clubs")]
pub struct DbClub {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub organization_id: Uuid,
    pub accent_color: Option<String>,
    pub background_color: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<i64>,
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

impl QueryDb<QueryablePlaceholder, SortableClub> for DbClub {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<&FilterConfig<QueryablePlaceholder>>,
    ) where
        Self: Sized,
    {
        todo!()
    }

    async fn query(
        pool: &PgPool,
        filter: Option<&FilterConfig<QueryablePlaceholder>>,
        sort: Option<&SortingConfig<SortableClub>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<Vec<Self>>
    where
        Self: BaseQuery + Sized,
    {
        todo!()
    }

    async fn response_pagination(
        pool: &sqlx::PgPool,
        filter: Option<&FilterConfig<QueryablePlaceholder>>,
        pagination: Option<&PaginationConfig>,
    ) -> Result<PaginationType>
    where
        Self: Sized,
    {
        todo!()
    }
}
