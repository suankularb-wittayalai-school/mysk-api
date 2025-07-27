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
use chrono::NaiveDate;
use futures::future;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailedCheerPracticePeriod {
    pub id: Uuid,
    pub date: NaiveDate,
    pub start_time: i64,
    pub duration: i64,
    pub delay: Option<i64>,
    pub classrooms: Vec<ClassroomWCheerAttendance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassroomWCheerAttendance {
    pub classroom: Classroom,
    pub count: i64,
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
        let attendance_count =
            DbCheerPracticePeriod::get_attendance_count_by_class(pool, relation.id, &classroom_ids)
                .await?;
        let futures = classroom_ids.iter().map(
            async |classroom_id| -> Result<Vec<CheerPracticeAttendance>> {
                let attendance_ids = DbCheerPracticeAttendance::get_by_classroom_id(
                    &mut *(pool.acquire().await?),
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
        .zip(attendance_count)
        .zip(attendances)
        .map(
            |((classroom, (_, count)), attendances)| ClassroomWCheerAttendance {
                classroom,
                count,
                attendances,
            },
        )
        .collect();

        Ok(Self {
            id: relation.id,
            date: relation.date,
            start_time: relation.start_time,
            duration: relation.duration,
            delay: relation.delay,
            classrooms,
        })
    }
}
