use actix_web::{get, web, HttpResponse, Responder};

use mysk_lib::{
    error::Error,
    models::{
        common::{
            requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
            response::ResponseType,
            traits::TopLevelGetById,
        },
        student::Student,
    },
};
use sqlx::types::Uuid;

use crate::AppState;

#[get("/{id}")]
pub async fn get_student_by_id(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    request_query: web::Query<RequestType<Student, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder, actix_web::Error> {
    let pool = &data.db;
    let student_id = id.into_inner();

    let fetch_level = request_query
        .fetch_level
        .as_ref()
        .unwrap_or(&FetchLevel::IdOnly);

    let descendant_fetch_level = request_query
        .descendant_fetch_level
        .as_ref()
        .unwrap_or(&FetchLevel::IdOnly);

    let student = Student::get_by_id(
        pool,
        student_id,
        Some(fetch_level),
        Some(descendant_fetch_level),
    )
    .await;

    match student {
        Ok(student) => Ok(HttpResponse::Ok().json(ResponseType::new(student, None))),
        Err(e) => {
            Ok(Error::EntityNotFound(e.to_string(), format!("/v1/students/{student_id}")).into())
        }
    }
}
