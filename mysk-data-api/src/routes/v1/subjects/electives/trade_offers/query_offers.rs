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

    Ok(HttpResponse::Ok())
}
