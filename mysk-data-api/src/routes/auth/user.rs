use actix_web::{get, HttpResponse, Responder};
use mysk_lib::models::common::response::ResponseType;

use crate::middlewares::logged_in::LoggedIn;

#[get("/user")]
async fn get_user(user: LoggedIn) -> Result<impl Responder, actix_web::Error> {
    Ok(HttpResponse::Ok().json(ResponseType::new(user, None)))
}
