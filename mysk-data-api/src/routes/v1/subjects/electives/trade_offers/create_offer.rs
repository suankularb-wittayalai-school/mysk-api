use crate::{extractors::student::LoggedInStudent, AppState};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::{
        elective_subject::request::{
            queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject,
        },
        elective_trade_offer::ElectiveTradeOffer,
    },
    prelude::*,
};
use uuid::Uuid;

#[post("")]
async fn create_trade_offer(
    data: Data<AppState>,
    trade_offer_id: Path<Uuid>,
    request_body: Json<
        RequestType<ElectiveTradeOffer, QueryableElectiveSubject, SortableElectiveSubject>,
    >,
    student_id: LoggedInStudent,
) -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}
