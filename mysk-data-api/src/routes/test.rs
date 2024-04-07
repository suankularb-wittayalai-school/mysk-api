use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};

use mysk_lib::models::*;
use mysk_lib::prelude::*;
use mysk_lib_macros::traits::db::GetById;
use uuid::Uuid;

use crate::AppState;

#[utoipa::path(path = "/test", tag = "Global")]
#[get("/test")]
pub async fn test(data: web::Data<AppState>, request: HttpRequest) -> Result<impl Responder> {
    let pool: &sqlx::PgPool = &data.db;

    let elective_id = Uuid::parse_str("aac285b1-15a5-4138-8b4d-88b743e472d3").unwrap();

    let elective = elective_subject::db::DbElectiveSubject::get_by_id(pool, elective_id).await?;

    let response = common::response::ResponseType::new(elective, None);

    Ok(HttpResponse::Ok().json(response))
}
