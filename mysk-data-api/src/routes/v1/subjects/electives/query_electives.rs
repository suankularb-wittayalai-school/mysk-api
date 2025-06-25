use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::{
        elective_subject::{
            ElectiveSubject,
            request::{queryable::QueryableElectiveSubject, sortable::SortableElectiveSubject},
        },
        traits::TopLevelQuery as _,
    },
    permissions::{Authorizer, roles::AdminRole},
    prelude::*,
};
use std::sync::Arc;

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
    }: RequestType<(), QueryableElectiveSubject, SortableElectiveSubject>,
) -> Result<impl Responder> {
    let pool = &data.db;
    // TODO: Fix later
    // let authorizer =
    //     Authorizer::new(pool, &user, "/subjects/electives".to_string()).await?;
    let authorizer = Authorizer::Admin(Arc::new(AdminRole));

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
