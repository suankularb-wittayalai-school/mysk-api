use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::response::ResponseType, models::certificate::db::DbCertificate, prelude::*,
};

#[get("/in-rsvp-period")]
pub async fn in_rsvp_period(
    data: Data<AppState>,
    _: ApiKeyHeader,
    _: LoggedInStudent,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;

    let is_in_rsvp_period = DbCertificate::is_rsvp_period(&mut conn).await?;
    let response = ResponseType::new(is_in_rsvp_period, None);

    Ok(HttpResponse::Ok().json(response))
}
