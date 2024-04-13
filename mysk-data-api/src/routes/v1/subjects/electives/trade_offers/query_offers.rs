use crate::{extractors::api_key::ApiKeyHeader, AppState};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::requests::RequestType,
    models::elective_trade_offer::{
        request::{queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer},
        ElectiveTradeOffer,
    },
    prelude::*,
};

#[get("")]
pub async fn query_trade_offers(
    data: Data<AppState>,
    // queryable and sortable ElectiveTradeOffer hasn't been implemented yet
    request_query: RequestType<
        ElectiveTradeOffer,
        QueryableElectiveTradeOffer,
        SortableElectiveTradeOffer,
    >,
    _api_key: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;

    Ok(HttpResponse::Ok())
}
