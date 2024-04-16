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
    common::{requests::RequestType, response::ResponseType},
    models::{
        elective_trade_offer::{
            request::{
                queryable::QueryableElectiveTradeOffer, sortable::SortableElectiveTradeOffer,
                updatable::UpdatableElectiveOffer,
            },
            ElectiveTradeOffer,
        },
        traits::TopLevelQuery as _,
    },
    prelude::*,
};

#[get("/{id}")]
pub async fn query_trade_offers(
    data: Data<AppState>,
    request_body: Json<
        RequestType<
            UpdatableElectiveOffer,
            QueryableElectiveTradeOffer,
            SortableElectiveTradeOffer,
        >,
    >,
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
