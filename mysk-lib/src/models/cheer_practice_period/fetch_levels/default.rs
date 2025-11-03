use crate::{
    common::requests::FetchLevel,
    models::{cheer_practice_period::db::DbCheerPracticePeriod, traits::FetchVariant},
    permissions::Authorizer,
    prelude::*,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultCheerPracticePeriod {
    pub id: Uuid,
    pub date: NaiveDate,
    pub start_time: i64,
    pub duration: i64,
    pub delay: Option<i64>,
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
            duration: relation.duration,
            delay: relation.delay,
            classrooms: classroom_ids,
        })
    }
}
