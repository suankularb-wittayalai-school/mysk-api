use crate::{
    AppState,
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
};
use actix_web::{HttpResponse, Responder, get, web::Data};
use mysk_lib::{
    common::{
        requests::{EmptyRequestData, RequestType},
        response::{MetadataType, ResponseType},
    },
    models::classroom::{
        Classroom,
        request::{queryable::QueryableClassroom, sortable::SortableClassroom},
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_classrooms(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<EmptyRequestData, QueryableClassroom, SortableClassroom>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let authorizer = Authorizer::new(&user, "/classrooms".to_string());

    let (classroom, pagination) = Classroom::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(classroom, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
