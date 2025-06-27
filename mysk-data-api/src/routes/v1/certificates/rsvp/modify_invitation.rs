use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
};
use actix_web::{
    HttpResponse, Responder, put,
    web::{Data, Json},
};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::ResponseType,
    },
    helpers::date::get_current_academic_year,
    models::{certificate::db::DbCertificate, enums::SubmissionStatus},
    prelude::*,
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
    }): Json<RequestType<ModifyInvitationRequest>>,
) -> Result<impl Responder> {
    let mut transaction = data.db.begin().await?;
    let rsvp_status = if matches!(request_data.rsvp_status, SubmissionStatus::Pending) {
        return Err(Error::InvalidRequest(
            "Status must be either `approved` or `declined`".to_string(),
            format!("/certificates/rsvp/{student_id}"),
        ));
    } else {
        request_data.rsvp_status
    };

    // Checks if the current time is within the rsvp period
    if !DbCertificate::is_rsvp_period(&mut transaction).await? {
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
