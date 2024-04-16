use crate::extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn};
use actix_web::{get, HttpResponse, Responder};
use mysk_lib::{common::response::ResponseType, prelude::*};

#[get("/user")]
async fn get_user(user: LoggedIn, _: ApiKeyHeader) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(ResponseType::new(user, None)))
}
