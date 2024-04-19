use actix_web::{HttpRequest, Responder};
use mysk_lib::prelude::*;

pub async fn not_found(req: HttpRequest) -> Result<impl Responder> {
    Err::<String, Error>(Error::EntityNotFound(
        "Route not found".to_string(),
        req.path().to_string(),
    ))
}
