use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    AppState,
};
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
    permissions,
    prelude::*,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn query_teacher_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    user: LoggedIn,
    id: Path<Uuid>,
    request_query: RequestType<Teacher, QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let user = user.0;
    let teacher_id = id.into_inner();
    let fetch_level = request_query.fetch_level.as_ref();
    let descendant_fetch_level = request_query.descendant_fetch_level.as_ref();
    let authorizer =
        permissions::get_authorizer(pool, &user, format!("/teachers/{teacher_id}")).await?;

    let teacher = Teacher::get_by_id(
        pool,
        teacher_id,
        fetch_level,
        descendant_fetch_level,
        &*authorizer,
    )
    .await?;
    let response = ResponseType::new(teacher, None);

    Ok(HttpResponse::Ok().json(response))
}
