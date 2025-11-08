use crate::{
    common::requests::FetchLevel,
    models::{cheer_practice_period::db::DbCheerPracticePeriod, traits::FetchVariant},
    permissions::Authorizer,
    prelude::*,
};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultCheerPracticePeriod {
    pub id: Uuid,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub delay: Option<i64>,
    pub note: Option<String>,
    pub classrooms: Vec<Uuid>,
}

impl FetchVariant for DefaultCheerPracticePeriod {
    type Relation = DbCheerPracticePeriod;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        _descendant_fetch_level: FetchLevel,
        _authorizer: &Authorizer,
    ) -> Result<Self> {
        // NOTE: classroom_ids can be returned directly because query_practice_periods forces an IdOnly descendant
        let classroom_ids =
            DbCheerPracticePeriod::get_classroom_ids(&mut *(pool.acquire().await?), relation.id)
                .await?;

        Ok(Self {
            id: relation.id,
            date: relation.date,
            start_time: relation.start_time,
            end_time: relation.end_time,
            delay: relation.delay,
            note: relation.note,
            classrooms: classroom_ids,
        })
    }
}
