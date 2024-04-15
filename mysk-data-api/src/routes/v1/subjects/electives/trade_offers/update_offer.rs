use crate::{
    extractors::{api_key::ApiKeyHeader, student::LoggedInStudent},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::elective_trade_offer::{
        request::updatable::UpdatableElectiveOffer, ElectiveTradeOffer,
    },
    prelude::*,
};
use uuid::Uuid;

#[put("/{id}")]
async fn update_trade_offer(
    data: Data<AppState>,
    trade_offer_id: Path<Uuid>,
    request_body: Json<
        RequestType<UpdatableElectiveOffer, QueryablePlaceholder, SortablePlaceholder>,
    >,
    student_id: LoggedInStudent,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;

    Ok(HttpResponse::Ok())
}
