use crate::{AppState, extractors::api_key::ApiKeyHeader};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::{EmptyRequestData, RequestType},
        response::{MetadataType, ResponseType},
    },
    models::elective_subject::{
        ElectiveSubject,
        request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
    },
    permissions::{Authorizer, roles::AdminRole},
    prelude::*,
};

#[get("")]
pub async fn query_elective_subject(
    data: Data<AppState>,
    _: ApiKeyHeader,
    // LoggedIn(user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<EmptyRequestData, QueryableElectiveSubject, SortableElectiveSubject>,
) -> Result<impl Responder> {
    let pool = &data.db;
    // TODO: Fix later
    // let mut conn = data.db.acquire().await?;
    // let authorizer =
    //     Authorizer::new(pool, &user, "/subjects/electives".to_string()).await?;
    let authorizer = Authorizer::Admin(AdminRole);

    let (electives, pagination) = ElectiveSubject::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(electives, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
