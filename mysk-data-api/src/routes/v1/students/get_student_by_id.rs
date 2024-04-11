use crate::{middlewares::api_key::HaveApiKey, AppState};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    models::{
        common::{
            requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
            traits::TopLevelGetById as _,
        },
        student::Student,
    },
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_student_by_id(
    data: Data<AppState>,
    id: Path<Uuid>,
    request_query: RequestType<Student, QueryablePlaceholder, SortablePlaceholder>,
    _: HaveApiKey,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let student = Student::get_by_id(pool, student_id, fetch_level, descendant_fetch_level).await?;

    Ok(HttpResponse::Ok().json(student))
}
