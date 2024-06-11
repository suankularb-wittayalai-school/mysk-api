use crate::{extractors::api_key::ApiKeyHeader, AppState};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::response::ResponseType, models::elective_subject::db::DbElectiveSubject, prelude::*,
};

#[get("/in-enrollment-period")]
pub async fn in_enrollment_period(data: Data<AppState>, _: ApiKeyHeader) -> Result<impl Responder> {
    let pool = &data.db;

    let is_in_enrollment_period = DbElectiveSubject::is_enrollment_period(pool).await?;
    let response = ResponseType::new(is_in_enrollment_period, None);

    Ok(HttpResponse::Ok().json(response))
}
