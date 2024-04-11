#![allow(unused_variables)]

use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::requests::RequestType,
    common::response::ResponseType,
    models::{
        elective_subject::{
            db::DbElectiveSubject,
            request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
            ElectiveSubject,
        },
        traits::QueryDb,
    },
    prelude::*,
};
use uuid::Uuid;

#[utoipa::path(path = "/test", tag = "Global")]
#[get("/test")]
pub async fn test(
    data: Data<AppState>,
    request_query: RequestType<ElectiveSubject, QueryableElectiveSubject, SortableElectiveSubject>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let model_id = Uuid::parse_str("d05c155a-ebe0-456b-b289-7b0eb1487f04").unwrap();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let filter = request_query.filter.as_ref();
    let sort = request_query.sort.as_ref();
    let pagination = request_query.pagination.as_ref();
    // let model = elective_subject::ElectiveSubject::get_by_id(
    //     pool,
    //     model_id,
    //     fetch_level,
    //     descendant_fetch_level,
    // )
    // .await?;

    let model = DbElectiveSubject::query(pool, filter, sort, pagination).await?;

    // let model = elective_trade_offer::ElectiveTradeOffer::get_by_id(
    //     pool,
    //     model_id,
    //     fetch_level,
    //     descendant_fetch_level,
    // )
    // .await?;

    let response = ResponseType::new(model, None);

    Ok(HttpResponse::Ok().json(response))
}
