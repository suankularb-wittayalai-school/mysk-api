use actix_web::{post, web, HttpResponse, Responder};
use mysk_lib::models::{auth::key::ApiKey, common::response::ResponseType};
use mysk_lib::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{middlewares::logged_in::LoggedIn, AppState};

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub expire_days: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateApiKeyResponse {
    pub api_key: String,
}

#[post("/keys")]
pub async fn create_api_key(
    data: web::Data<AppState>,
    query: web::Json<CreateApiKeyRequest>,
    user: LoggedIn,
) -> Result<impl Responder> {
    let pool: &sqlx::Pool<sqlx::Postgres> = &data.db;

    let api_key = ApiKey::create(pool, user.0.id, query.expire_days).await?;

    let response: ResponseType<CreateApiKeyResponse> = ResponseType::new(
        CreateApiKeyResponse {
            api_key: api_key.to_string(),
        },
        None,
    );

    Ok(HttpResponse::Ok().json(response))
}
