use crate::{middlewares::logged_in::LoggedIn, AppState};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mysk_lib::{
    models::{auth::key::ApiKey, common::response::ResponseType},
    prelude::*,
};
use serde::{Deserialize, Serialize};

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
    data: Data<AppState>,
    query: Json<CreateApiKeyRequest>,
    user: LoggedIn,
) -> Result<impl Responder> {
    let pool = &data.db;
    let api_key = ApiKey::create(pool, user.0.id, query.expire_days).await?;

    let response: ResponseType<CreateApiKeyResponse> = ResponseType::new(
        CreateApiKeyResponse {
            api_key: api_key.to_string(),
        },
        None,
    );

    Ok(HttpResponse::Ok().json(response))
}
