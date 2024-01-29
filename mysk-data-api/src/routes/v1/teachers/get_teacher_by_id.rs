use actix_web::{get, web, Responder};
use sqlx::types::Uuid;

use mysk_lib::models::{
    common::{
        requests::{FetchLevel, QueryablePlaceholder, RequestType, SortablePlaceholder},
        traits::TopLevelGetById,
    },
    teacher::Teacher,
};
use mysk_lib::prelude::*;

use crate::{middlewares::api_key::HaveApiKey, AppState};

#[get("/{id}")]
pub async fn get_teacher_by_id(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    _: HaveApiKey,
    request_query: web::Query<RequestType<Teacher, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool: &sqlx::Pool<sqlx::Postgres> = &data.db;
    let teacher_id = id.into_inner();

    let fetch_level = request_query
        .fetch_level
        .as_ref()
        .unwrap_or(&FetchLevel::IdOnly);

    let descendant_fetch_level = request_query
        .descendant_fetch_level
        .as_ref()
        .unwrap_or(&FetchLevel::IdOnly);

    let teacher = Teacher::get_by_id(
        pool,
        teacher_id,
        Some(fetch_level),
        Some(descendant_fetch_level),
    )
    .await?;

    Ok(teacher)
}
