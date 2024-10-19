use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::response::ResponseType, models::elective_subject::db::DbElectiveSubject, prelude::*,
};

#[get("/in-enrollment-period")]
pub async fn in_enrollment_period(
    data: Data<AppState>,
    _: ApiKeyHeader,
    student_id: LoggedInStudent,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = student_id.0;

    let is_in_enrollment_period = DbElectiveSubject::is_enrollment_period(pool, student_id).await?;
    let response = ResponseType::new(is_in_enrollment_period, None);

    Ok(HttpResponse::Ok().json(response))
}
