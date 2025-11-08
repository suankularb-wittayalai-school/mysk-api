use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::response::ResponseType, models::cheer_practice_period::db::DbCheerPracticePeriod,
    prelude::*,
};

#[get("/in-jaturamitr-period")]
pub async fn in_jaturamitr_period(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(_): LoggedIn,
) -> Result<impl Responder> {
    let mut conn = data.db.acquire().await?;

    let is_in_jaturamitr_period = DbCheerPracticePeriod::in_jaturamitr_period(&mut conn).await?;
    let response = ResponseType::new(is_in_jaturamitr_period, None);

    Ok(HttpResponse::Ok().json(response))
}
