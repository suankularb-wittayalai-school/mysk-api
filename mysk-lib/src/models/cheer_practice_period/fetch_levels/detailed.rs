use crate::{
    common::requests::FetchLevel,
    models::{
        cheer_practice_attendance::{CheerPracticeAttendance, db::DbCheerPracticeAttendance},
        cheer_practice_period::db::DbCheerPracticePeriod,
        classroom::Classroom,
        traits::FetchVariant,
    },
    permissions::Authorizer,
    prelude::*,
};
use chrono::{NaiveDate, NaiveTime};
use futures::future;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailedCheerPracticePeriod {
    pub id: Uuid,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub delay: Option<i64>,
    pub note: Option<String>,
    pub classrooms: Vec<ClassroomWCheerAttendance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassroomWCheerAttendance {
    pub classroom: Classroom,
    pub attendances: Vec<CheerPracticeAttendance>,
}

impl FetchVariant for DetailedCheerPracticePeriod {
    type Relation = DbCheerPracticePeriod;

    async fn from_relation(
        pool: &PgPool,
        relation: Self::Relation,
        descendant_fetch_level: FetchLevel,
        authorizer: &Authorizer,
    ) -> Result<Self> {
        let classroom_ids =
            DbCheerPracticePeriod::get_classroom_ids(&mut *(pool.acquire().await?), relation.id)
                .await?;
        let futures = classroom_ids.iter().map(
            async |classroom_id| -> Result<Vec<CheerPracticeAttendance>> {
                let attendance_ids = DbCheerPracticeAttendance::get_by_classroom_id(
                    pool,
                    relation.id,
                    *classroom_id,
                )
                .await?;

                CheerPracticeAttendance::get_by_ids(
                    pool,
                    &attendance_ids,
                    descendant_fetch_level,
                    FetchLevel::IdOnly,
                    authorizer,
                )
                .await
            },
        );
        let attendances = future::try_join_all(futures)
            .await?
            .into_iter()
            .collect::<Vec<_>>();
        let classrooms = Classroom::get_by_ids(
            pool,
            &classroom_ids,
            descendant_fetch_level,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?
        .into_iter()
        .zip(attendances)
        .map(|(classroom, attendances)| ClassroomWCheerAttendance {
            classroom,
            attendances,
        })
        .collect();

        Ok(Self {
            id: relation.id,
            date: relation.date,
            start_time: relation.start_time,
            end_time: relation.end_time,
            delay: relation.delay,
            note: relation.note,
            classrooms,
        })
    }
}
