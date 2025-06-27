use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::response::ResponseType, models::certificate::db::DbCertificate, prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_invitation_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedInStudent(client_student_id): LoggedInStudent,
    student_id: Path<Uuid>,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;
    let student_id = student_id.into_inner();

    // TODO: too lazy to write authorizer right now (27/02/25)
    // Only allow same student ID query
    if client_student_id != student_id {
        return Err(Error::InvalidPermission(
            "Insufficient permissions to perform this action".to_string(),
            format!("/certificates/rsvp/{student_id}"),
        ));
    }

    let rsvp_status = DbCertificate::get_rsvp_status(&mut conn, student_id).await?;
    let response = ResponseType::new(rsvp_status, None);

    Ok(HttpResponse::Ok().json(response))
}
