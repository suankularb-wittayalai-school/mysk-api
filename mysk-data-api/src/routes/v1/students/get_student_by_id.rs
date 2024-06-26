use crate::{extractors::api_key::ApiKeyHeader, AppState};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{QueryablePlaceholder, RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::{student::Student, traits::TopLevelGetById as _},
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_student_by_id(
    data: Data<AppState>,
    _: ApiKeyHeader,
    id: Path<Uuid>,
    request_query: RequestType<Student, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let student_id = id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let student = Student::get_by_id(pool, student_id, fetch_level, descendant_fetch_level).await?;
    let response = ResponseType::new(student, None);

    Ok(HttpResponse::Ok().json(response))
}
