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
    models::{elective_subject::ElectiveSubject, traits::TopLevelGetById as _},
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_elective_details(
    data: Data<AppState>,
    elective_subject_session_id: Path<Uuid>,
    request_query: RequestType<ElectiveSubject, QueryablePlaceholder, SortablePlaceholder>,
    _: ApiKeyHeader,
) -> Result<impl Responder> {
    let pool = &data.db;
    let elective_subject_session_id = elective_subject_session_id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let elective_subject = ElectiveSubject::get_by_id(
        pool,
        elective_subject_session_id,
        fetch_level,
        descendant_fetch_level,
    )
    .await?;
    let response = ResponseType::new(elective_subject, None);

    Ok(HttpResponse::Ok().json(response))
}
