use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use mysk_lib::{
    common::{requests::RequestType, response::ResponseType},
    models::elective_subject::ElectiveSubject,
    permissions::{Authorizer, roles::AdminRole},
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_elective_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    // LoggedIn(user): LoggedIn,
    elective_subject_session_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType,
) -> Result<impl Responder> {
    let pool = &data.db;
    let elective_subject_session_id = elective_subject_session_id.into_inner();
    // TODO: Fix later
    // let mut conn = data.db.acquire().await?;
    // let authorizer = Authorizer::new(
    //     pool,
    //     &user,
    //     format!("/subjects/electives/{elective_subject_session_id}"),
    // )
    // .await?;
    let authorizer = Authorizer::Admin(AdminRole);

    let elective_subject = ElectiveSubject::get_by_id(
        pool,
        elective_subject_session_id,
        fetch_level,
        descendant_fetch_level,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(elective_subject, None);

    Ok(HttpResponse::Ok().json(response))
}
