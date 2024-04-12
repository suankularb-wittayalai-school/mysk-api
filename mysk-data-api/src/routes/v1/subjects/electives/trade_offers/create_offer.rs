use actix_web::{post, HttpResponse, Responder};
use mysk_lib::{common::response::ResponseType, prelude::*};

#[post("")]
async fn create_trade_offer() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}
