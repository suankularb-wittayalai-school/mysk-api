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
    models::student::{
        Student,
        request::{queryable::QueryableStudent, sortable::SortableStudent},
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_students(
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
    }: RequestType<(), QueryableStudent, SortableStudent>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&mut conn, &user, "/students".to_string()).await?;

    let (student, pagination) = Student::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(student, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
