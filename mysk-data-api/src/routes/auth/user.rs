use actix_web::{get, HttpResponse, Responder};

use mysk_lib::models::common::response::ResponseType;
use mysk_lib::prelude::*;

use crate::middlewares::{api_key::HaveApiKey, logged_in::LoggedIn};

#[get("/user")]
async fn get_user(user: LoggedIn, _api_key: HaveApiKey) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(ResponseType::new(user, None)))
}
