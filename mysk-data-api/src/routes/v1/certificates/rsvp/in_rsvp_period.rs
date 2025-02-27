use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::response::ResponseType, models::certificate::db::DbCertificate, prelude::*,
};

#[get("/in-rsvp-period")]
pub async fn in_rsvp_period(
    data: Data<AppState>,
    _: ApiKeyHeader,
    _: LoggedInStudent,
) -> Result<impl Responder> {
    let pool = &data.db;

    let is_in_rsvp_period = DbCertificate::is_rsvp_period(pool).await?;
    let response = ResponseType::new(is_in_rsvp_period, None);

    Ok(HttpResponse::Ok().json(response))
}
