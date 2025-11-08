use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{common::response::ResponseType, helpers::date::is_today_jaturamitr, prelude::*};

#[get("/in-jaturamitr-period")]
pub async fn in_jaturamitr_period(
    _data: Data<AppState>,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let is_in_jaturamitr_period = is_today_jaturamitr();

    let response = ResponseType::new(is_in_jaturamitr_period, None);

    Ok(HttpResponse::Ok().json(response))
}
