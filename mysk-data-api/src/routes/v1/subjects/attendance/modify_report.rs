use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, teacher::LoggedInTeacher},
};
use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json, Path},
};
use chrono::NaiveDate;
use mysk_lib::{
    common::{
        requests::RequestType,
        response::ResponseType,
    },
    models::{
        online_teaching_reports::{OnlineTeachingReports, db::DbOnlineTeachingReports},
        traits::GetById as _,
    },
    permissions::Authorizer,
    prelude::*,
    query::{QueryParam, SqlSetClause},
};
use serde::Deserialize;
use sqlx::Error as SqlxError;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdateReportRequest {
    subject_id: Option<Uuid>,
    classroom_id: Option<Uuid>,
    date: Option<NaiveDate>,
    teaching_methods: Option<Vec<String>>,
    teaching_topic: Option<String>,
    suggestions: Option<String>,
    start_time: Option<i64>,
    duration: Option<i64>,
    absent_student_no: Option<String>,
}

#[put("/{id}")]
pub async fn modify_report(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    LoggedInTeacher(teacher_id): LoggedInTeacher,
    report_id: Path<Uuid>,
    Json(RequestType {
        data: update_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<UpdateReportRequest>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let report_id = report_id.into_inner();
    let authorizer = Authorizer::new(
        &mut conn,
        &user,
        format!("/subjects/attendance/{report_id}"),
    )
    .await?;

    // Check if class report exists
    let report_id = DbOnlineTeachingReports::get_by_id(&mut conn, report_id)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => Error::EntityNotFound(
                "Class report not found".to_string(),
                format!("/subjects/attendance/{report_id}"),
            ),
            _ => e.into(),
        })?
        .id;

    let mut qb = SqlSetClause::new();
    qb.push_update_field("subject_id", update_data.subject_id, QueryParam::Uuid)
        .push_update_field("classroom_id", update_data.classroom_id, QueryParam::Uuid)
        .push_update_field("date", update_data.date, QueryParam::NaiveDate)
        .push_update_field(
            "teaching_methods",
            update_data.teaching_methods,
            QueryParam::ArrayString,
        )
        .push_update_field(
            "teaching_topic",
            update_data.teaching_topic,
            QueryParam::String,
        )
        .push_update_field("suggestions", update_data.suggestions, QueryParam::String)
        .push_update_field("start_time", update_data.start_time, QueryParam::Int)
        .push_update_field("duration", update_data.duration, QueryParam::Int)
        .push_update_field(
            "absent_student_no",
            update_data.absent_student_no,
            QueryParam::String,
        );

    let mut qb = qb.into_query_builder("UPDATE online_teaching_reports");
    qb.push(" WHERE id = ")
        .push_bind(report_id)
        .push(" AND teacher_id = ")
        .push_bind(teacher_id)
        .build()
        .execute(&mut *conn)
        .await?;

    let class_report = OnlineTeachingReports::get_by_id(
        pool,
        report_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(class_report, None);

    Ok(HttpResponse::Ok().json(response))
}
