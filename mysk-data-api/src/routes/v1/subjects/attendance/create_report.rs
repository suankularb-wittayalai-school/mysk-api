use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, teacher::LoggedInTeacher},
};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use chrono::NaiveDate;
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    helpers::date::get_current_date,
    models::{
        classroom::db::DbClassroom, online_teaching_reports::OnlineTeachingReports,
        subject::db::DbSubject, traits::GetById as _,
    },
    permissions::Authorizer,
    prelude::*,
};
use serde::Deserialize;
use sqlx::{Error as SqlxError, query};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateReportRequest {
    subject_id: Uuid,
    classroom_id: Option<Uuid>,
    date: Option<NaiveDate>,
    teaching_methods: Vec<String>,
    teaching_topic: String,
    suggestions: Option<String>,
    start_time: i64,
    duration: i64,
    absent_student_no: Option<String>,
}

#[post("")]
pub async fn create_report(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInTeacher(teacher_id): LoggedInTeacher,
    Json(RequestType {
        data: class_report,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<CreateReportRequest>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&user, "/subjects/attendance".to_string());

    // Check if subject exists
    let subject_id = DbSubject::get_by_id(&mut conn, class_report.subject_id)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => Error::EntityNotFound(
                "Subject not found".to_string(),
                "/subjects/attendance".to_string(),
            ),
            _ => e.into(),
        })?
        .id;

    // Check if classroom exists
    let classroom_id = if let Some(classroom_id) = class_report.classroom_id {
        Some(
            DbClassroom::get_by_id(&mut conn, classroom_id)
                .await
                .map_err(|e| match e {
                    SqlxError::RowNotFound => Error::EntityNotFound(
                        "Classroom not found".to_string(),
                        "/subjects/attendance".to_string(),
                    ),
                    _ => e.into(),
                })?
                .id,
        )
    } else {
        None
    };

    if class_report.teaching_methods.is_empty() {
        return Err(Error::InvalidRequest(
            "At least one teaching method must be provided".to_string(),
            "/subjects/attendance".to_string(),
        ));
    }

    let new_class_report_id = query!(
        "
        INSERT INTO online_teaching_reports\
        (subject_id, teacher_id, classroom_id, date, teaching_methods, teaching_topic, suggestions,\
        start_time, duration, absent_student_no)\
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id\
        ",
        subject_id,
        teacher_id,
        classroom_id,
        class_report.date.unwrap_or(get_current_date()),
        &class_report.teaching_methods[..],
        class_report.teaching_topic,
        class_report.suggestions,
        class_report.start_time,
        class_report.duration,
        class_report.absent_student_no,
    )
    .fetch_one(&mut *conn)
    .await?
    .id;

    let new_class_report = OnlineTeachingReports::get_by_id(
        pool,
        new_class_report_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(new_class_report, None);

    Ok(HttpResponse::Ok().json(response))
}
