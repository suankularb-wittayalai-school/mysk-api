use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::elective_trade_offer::ElectiveTradeOffer,
    prelude::*,
};

#[get("")]
pub async fn query_trade_offers(
    data: Data<AppState>,
    request_body: Json<RequestType<ElectiveTradeOffer, QueryablePlaceholder, SortablePlaceholder>>,
    student_id: LoggedInStudent,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    // let fetch_level = request_body.fetch_level.as_ref();
    // let descendant_fetch_level = request_body.descendant_fetch_level.as_ref();
    // let filter = request_body.filter.as_ref();
    // let sort = request_body.sort.as_ref();
    // let pagination = request_body.pagination.as_ref();

    Ok(HttpResponse::Ok())
}
