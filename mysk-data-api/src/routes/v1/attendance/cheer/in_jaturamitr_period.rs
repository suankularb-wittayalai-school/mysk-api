use std::{collections::HashSet, sync::LazyLock};

use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use chrono::NaiveDate;
use mysk_lib::{common::response::ResponseType, prelude::*};

static JATURAMITR_DATES: LazyLock<HashSet<NaiveDate>> = LazyLock::new(|| {
    HashSet::from([
        NaiveDate::from_ymd_opt(2025, 11, 13).unwrap(),
        NaiveDate::from_ymd_opt(2025, 11, 15).unwrap(),
        NaiveDate::from_ymd_opt(2025, 11, 22).unwrap(),
    ])
});

#[get("/in-jaturamitr-period/{date}")]
pub async fn in_jaturamitr_period(
    _data: Data<AppState>,
    _: ApiKeyHeader,
    date: Path<NaiveDate>,
) -> Result<impl Responder> {
    let response = ResponseType::new(JATURAMITR_DATES.contains(&date.into_inner()), None);

    Ok(HttpResponse::Ok().json(response))
}
