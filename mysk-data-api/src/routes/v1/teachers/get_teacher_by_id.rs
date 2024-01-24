use actix_web::{get, web, HttpResponse, Responder};

use mysk_lib::{
    error::Error,
    models::{
        common::{
            requests::{FetchLevel, RequestType},
            response::ResponseType,
            traits::TopLevelGetById,
        },
        teacher::Teacher,
    },
};
use sqlx::types::Uuid;

use crate::AppState;

// TODO: change this to a real type
#[derive(Debug, serde::Deserialize)]
pub struct Placeholder;

#[get("/{id}")]
pub async fn get_teacher_by_id(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    request_query: web::Query<RequestType<Teacher, Placeholder, Placeholder>>,
) -> Result<impl Responder, actix_web::Error> {
    let pool: &sqlx::Pool<sqlx::Postgres> = &data.db;
    let teacher_id = id.into_inner();

    let fetch_level = request_query
        .fetch_level
        .as_ref()
        .unwrap_or(&FetchLevel::IdOnly);

    let descendant_fetch_level = request_query
        .descendant_fetch_level
        .as_ref()
        .unwrap_or(&FetchLevel::IdOnly);

    let student = Teacher::get_by_id(
        pool,
        teacher_id,
        Some(fetch_level),
        Some(descendant_fetch_level),
    )
    .await;

    match student {
        Ok(student) => Ok(HttpResponse::Ok().json(ResponseType::new(student, None))),
        Err(e) => {
            Ok(Error::EntityNotFound(e.to_string(), format!("/v1/teachers/{teacher_id}")).into())
        }
    }
}
