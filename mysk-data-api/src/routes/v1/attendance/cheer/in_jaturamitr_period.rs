use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::response::ResponseType, models::cheer_practice_period::db::DbCheerPracticePeriod,
    prelude::*,
};
use uuid::Uuid;

#[get("/in-jaturamitr-period/{id}")]
pub async fn in_jaturamitr_period(
    _data: Data<AppState>,
    _: ApiKeyHeader,
    practice_period_id: Path<Uuid>,
) -> Result<impl Responder> {
    let is_in_jaturamitr_period =
        DbCheerPracticePeriod::in_jaturamitr_period(practice_period_id.into_inner());

    let response = ResponseType::new(is_in_jaturamitr_period, None);

    Ok(HttpResponse::Ok().json(response))
}
