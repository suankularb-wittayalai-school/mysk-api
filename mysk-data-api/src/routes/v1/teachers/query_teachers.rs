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
        teacher::{
            Teacher,
            request::{queryable::QueryableTeacher, sortable::SortableTeacher},
        },
        traits::TopLevelQuery as _,
    },
    permissions::Authorizer,
    prelude::*,
};

#[get("")]
pub async fn query_teachers(
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
    }: RequestType<(), QueryableTeacher, SortableTeacher>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let mut conn = data.db.acquire().await?;
    let authorizer = Authorizer::new(&mut conn, &user, "/teachers".to_string()).await?;

    let (teacher, pagination) = Teacher::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &authorizer,
    )
    .await?;
    let response = ResponseType::new(teacher, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
