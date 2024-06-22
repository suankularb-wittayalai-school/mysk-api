use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::response::ResponseType, helpers::date::get_current_academic_year, prelude::*,
};
use serde::Serialize;
use sqlx::{query, PgPool};

#[derive(Debug, Serialize)]
struct ClubStatistics {
    pub club_members: i64,
    pub club_staffs: i64,
    pub active_clubs: i64,
    pub total_students: i64,
}

impl ClubStatistics {
    pub async fn new(pool: &PgPool) -> Result<Self> {
        let current_year = get_current_academic_year(None);

        let club_members = query!(
            "SELECT COUNT(DISTINCT student_id) as count FROM club_members WHERE year = $1",
            current_year,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);

        let club_staffs = query!(
            "SELECT COUNT(DISTINCT student_id) as count FROM club_staffs WHERE year = $1",
            current_year,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);

        let active_clubs = query!(
            "SELECT COUNT(DISTINCT club_id) as count FROM club_staffs WHERE year = $1",
            current_year,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);

        // Total students for percentage calculation
        let total_students = query!(
            "SELECT count(DISTINCT students.id) as count FROM students
            JOIN classroom_students ON classroom_students.student_id = students.id
            JOIN classrooms ON classrooms.id = classroom_students.classroom_id
            AND classrooms.year = $1",
            current_year,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);

        Ok(ClubStatistics {
            club_members,
            club_staffs,
            active_clubs,
            total_students,
        })
    }
}

#[get("/statistics")]
async fn get_club_statistics(data: Data<AppState>) -> Result<impl Responder> {
    let pool = &data.db;
    let statistics = ClubStatistics::new(pool).await?;
    let response = ResponseType::new(statistics, None);

    Ok(HttpResponse::Ok().json(response))
}
