use crate::{
    common::requests::FetchLevel,
    models::{
        cheer_practice_period::db::DbCheerPracticePeriod,
        classroom::{Classroom, db::DbClassroom},
        student::Student,
        traits::FetchVariant,
    },
    permissions::Authorizer,
    prelude::*,
};
use chrono::NaiveDate;
use futures::future;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetailedCheerPracticePeriod {
    pub id: Uuid,
    pub date: NaiveDate,
    pub start_time: i64,
    pub duration: i64,
    pub delay: Option<i64>,
    pub classrooms: Vec<Classroom>,
    pub students: HashMap<Uuid, Vec<Student>>,
    pub attendance_count: HashMap<Uuid, i64>,
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
        let classrooms = Classroom::get_by_ids(
            pool,
            &classroom_ids,
            descendant_fetch_level,
            FetchLevel::IdOnly,
            authorizer,
        )
        .await?;
        let futures =
            classroom_ids
                .iter()
                .map(async |classroom_id| -> Result<(Uuid, Vec<Student>)> {
                    let student_ids = DbClassroom::get_classroom_students(
                        &mut *(pool.acquire().await?),
                        relation.id,
                    )
                    .await?;
                    Ok((
                        *classroom_id,
                        Student::get_by_ids(
                            pool,
                            &student_ids,
                            descendant_fetch_level,
                            FetchLevel::IdOnly,
                            authorizer,
                        )
                        .await?,
                    ))
                });
        let students = future::try_join_all(futures).await?.into_iter().collect();
        let attendance_count =
            DbCheerPracticePeriod::get_attendance_count_by_class(pool, relation.id, &classroom_ids)
                .await?;

        Ok(Self {
            id: relation.id,
            date: relation.date,
            start_time: relation.start_time,
            duration: relation.duration,
            delay: relation.delay,
            classrooms,
            students,
            attendance_count,
        })
    }
}
