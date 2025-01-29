use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::response::ResponseType, models::elective_subject::db::DbElectiveSubject, prelude::*,
};

#[get("/previously-enrolled")]
async fn get_previously_enrolled(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedInStudent(student_id): LoggedInStudent,
) -> Result<impl Responder> {
    let pool = &data.db;

    let electives = DbElectiveSubject::get_previously_enrolled_electives(pool, student_id).await?;
    let response = ResponseType::new(electives, None);

    Ok(HttpResponse::Ok().json(response))
}
