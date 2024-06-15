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
    models::{teacher::Teacher, traits::TopLevelGetById as _},
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_teacher_by_id(
    data: Data<AppState>,
    _: ApiKeyHeader,
    id: Path<Uuid>,
    request_query: RequestType<Teacher, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let teacher_id = id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let teacher = Teacher::get_by_id(pool, teacher_id, fetch_level, descendant_fetch_level).await?;
    let response = ResponseType::new(teacher, None);

    Ok(HttpResponse::Ok().json(response))
}
