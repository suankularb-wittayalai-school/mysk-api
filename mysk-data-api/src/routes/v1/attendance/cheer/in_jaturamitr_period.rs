use std::collections::HashSet;

use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use chrono::NaiveDate;
use mysk_lib::{common::response::ResponseType, prelude::*};

#[get("/in-jaturamitr-period/{date}")]
pub async fn in_jaturamitr_period(
    _data: Data<AppState>,
    _: ApiKeyHeader,
    date: Path<NaiveDate>,
) -> Result<impl Responder> {
    let jaturamitr_dates = HashSet::from([
        NaiveDate::from_ymd_opt(2025, 11, 13).unwrap(),
        NaiveDate::from_ymd_opt(2025, 11, 15).unwrap(),
        NaiveDate::from_ymd_opt(2025, 11, 22).unwrap(),
    ]);

    let is_jaturamitr_day = if jaturamitr_dates.contains(&date.into_inner()) {
        true
    } else {
        false
    };

    let response = ResponseType::new(is_jaturamitr_day, None);

    Ok(HttpResponse::Ok().json(response))
}
