use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn, teacher::LoggedInTeacher},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use mysk_lib::{
    common::{
        requests::{QueryParam, QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{
        online_teaching_reports::{db::DbOnlineTeachingReports, OnlineTeachingReports},
        traits::TopLevelGetById as _,
    },
    permissions,
    prelude::*,
    query::set_clause::SqlSetClause,
};
use mysk_lib_macros::traits::db::GetById as _;
use serde::Deserialize;
use sqlx::Error as SqlxError;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct UpdateClassReportRequest {
    subject_id: Option<Uuid>,
    classroom_id: Option<Uuid>,
    date: Option<NaiveDate>,
    teaching_methods: Option<Vec<String>>,
    teaching_topic: Option<String>,
    suggestions: Option<String>,
}

#[put("/{id}")]
pub async fn modify_class_report(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    teacher_id: LoggedInTeacher,
    report_id: Path<Uuid>,
    Json(request_body): Json<
        RequestType<UpdateClassReportRequest, QueryablePlaceholder, SortablePlaceholder>,
    >,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let teacher_id = teacher_id.0;
    let report_id = report_id.into_inner();
    let Some(update_data) = request_body.data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/subjects/attendance/{report_id}"),
        ));
    };
    let fetch_level = request_body.fetch_level.as_ref();
    let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/subjects/attendance/{report_id}"))
            .await?;

    // Check if class report exists
    let report_id = DbOnlineTeachingReports::get_by_id(pool, report_id)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => Error::EntityNotFound(
                "Class report not found".to_string(),
                format!("/subjects/attendance/{report_id}"),
            ),
            _ => e.into(),
        })?
        .id;

    // TODO: 0.6.0 rebase :skull_crossbones:
    let mut qb = SqlSetClause::new()
        .push_update_field("subject_id", update_data.subject_id, QueryParam::Uuid)
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
        .into_query_builder("UPDATE online_teaching_reports");

    qb.push(" WHERE id = ")
        .push_bind(report_id)
        .push(" AND teacher_id = ")
        .push_bind(teacher_id)
        .build()
        .execute(pool)
        .await?;

    let class_report = OnlineTeachingReports::get_by_id(
        pool,
        report_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(class_report, None);

    Ok(HttpResponse::Ok().json(response))
}
