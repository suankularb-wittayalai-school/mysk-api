use crate::{middlewares::api_key::HaveApiKey, AppState};
use actix_web::{get, web::{Data, Path}, HttpResponse, Responder};
use mysk_lib::{
    models::{
        common::{
            requests::{
                QueryablePlaceholder, RequestType, SortablePlaceholder,
            },
            response::ResponseType,
            traits::TopLevelGetById as _,
        },
        elective_subject::ElectiveSubject,
    },
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_elective_details(
    data: Data<AppState>,
    path: Path<Uuid>,
    request_query: RequestType<
        ElectiveSubject,
        QueryablePlaceholder,
        SortablePlaceholder,
    >,
    _: HaveApiKey,
) -> Result<impl Responder> {
    let pool = &data.db;
    let id = path.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();

    let elective_subject = ElectiveSubject::get_by_id(
        pool,
        id,
        fetch_level,
        descendant_fetch_level,
    )
    .await?;
    let response = ResponseType::new(elective_subject, None);

    Ok(HttpResponse::Ok().json(response))
}
