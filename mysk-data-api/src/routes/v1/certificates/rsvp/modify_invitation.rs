use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    helpers::date::get_current_academic_year,
    models::{certificate::db::DbCertificate, enums::SubmissionStatus},
    prelude::*,
    query::QueryablePlaceholder,
};
use serde::Deserialize;
use sqlx::query;

#[derive(Deserialize)]
struct ModifyInvitationRequest {
    rsvp_status: SubmissionStatus,
}

#[put("")]
async fn modify_invitation(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedInStudent(student_id): LoggedInStudent,
    Json(RequestType {
        data: request_data, ..
    }): Json<RequestType<ModifyInvitationRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut transaction = pool.begin().await?;
    let rsvp_status = if let Some(request_data) = request_data {
        if matches!(request_data.rsvp_status, SubmissionStatus::Pending) {
            return Err(Error::InvalidRequest(
                "Status must be either `approved` or `declined`".to_string(),
                format!("/certificates/rsvp/{student_id}"),
            ));
        }

        request_data.rsvp_status
    } else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/certificates/rsvp/{student_id}"),
        ));
    };

    // Checks if the current time is within the rsvp period
    if !DbCertificate::is_rsvp_period(&mut *transaction).await? {
        return Err(Error::InvalidPermission(
            "The certificate ceremony RSVP period has ended".to_string(),
            format!("/certificates/rsvp/{student_id}"),
        ));
    }

    query!(
        "UPDATE student_certificates SET rsvp_status = $1 WHERE student_id = $2 AND year = $3",
        rsvp_status as SubmissionStatus,
        student_id,
        get_current_academic_year(None),
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    // TODO: Data models
    let response = ResponseType::new(true, None);

    Ok(HttpResponse::Ok().json(response))
}
