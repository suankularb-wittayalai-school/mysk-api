use crate::{extractors::student::LoggedInStudent, AppState};
use actix_web::{
    put,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{common::response::ResponseType, prelude::*};
use uuid::Uuid;

#[put("/{id}")]
async fn update_trade_offer(
    data: Data<AppState>,
    trade_offer_id: Path<Uuid>,
    student_id: LoggedInStudent,
) -> Result<impl Responder> {
    let pool = &data.db;

    Ok(HttpResponse::Ok())
}
