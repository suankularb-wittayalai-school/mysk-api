use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::response::ResponseType, models::elective_subject::db::DbElectiveSubject, prelude::*,
};

#[get("/previously-enrolled")]
async fn get_previously_enrolled(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedInStudent(student_id): LoggedInStudent,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;

    let electives =
        DbElectiveSubject::get_previously_enrolled_electives(&mut conn, student_id).await?;
    let response = ResponseType::new(electives, None);

    Ok(HttpResponse::Ok().json(response))
}
