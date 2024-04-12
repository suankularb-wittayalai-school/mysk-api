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
    models::elective_subject::ElectiveSubject,
    prelude::*,
};

#[get("/{session_code}")]
pub async fn query_elective_details(
    data: Data<AppState>,
    path: Path<i64>,
    request_query: RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>,
    _api_key: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let id = path.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let elective_subject =
        ElectiveSubject::get_by_session_code(pool, id, fetch_level, descendant_fetch_level).await?;
    let response = ResponseType::new(elective_subject, None);

    Ok(HttpResponse::Ok().json(response))
}
