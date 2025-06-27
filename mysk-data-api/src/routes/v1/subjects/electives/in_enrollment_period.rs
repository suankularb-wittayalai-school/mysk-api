use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::response::ResponseType, models::elective_subject::db::DbElectiveSubject, prelude::*,
};

#[get("/in-enrollment-period")]
pub async fn in_enrollment_period(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedInStudent(student_id): LoggedInStudent,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;

    let is_in_enrollment_period =
        DbElectiveSubject::is_enrollment_period(&mut conn, student_id).await?;
    let response = ResponseType::new(is_in_enrollment_period, None);

    Ok(HttpResponse::Ok().json(response))
}
