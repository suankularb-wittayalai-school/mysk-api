use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
use chrono::{SecondsFormat, Utc};
use mysk_lib::{common::response::ResponseType, prelude::*};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
struct HealthCheckResponse {
    server_time: String,
    database_connection: bool,
    database_response_time: u128,
}

impl HealthCheckResponse {
    pub async fn new(pool: &PgPool) -> Self {
        let start = time::Instant::now();

        let database_connection = sqlx::query("SELECT 1").execute(pool).await.is_ok();
        let database_response_time = start.elapsed().as_millis();

        HealthCheckResponse {
            server_time: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            database_connection,
            database_response_time,
        }
    }
}

#[utoipa::path(
    responses(
        (status=200, body=HealthCheckResponse, description="The server is healthy"),
        (status=500, body=String, description="The server is not healthy")
    ),
    path="/health-check",
    tag="Global"
)]
#[get("/health-check")]
pub async fn health_check(data: Data<AppState>) -> Result<impl Responder> {
    let pool = &data.db;
    let health_check_response = HealthCheckResponse::new(pool).await;
    let response: ResponseType<HealthCheckResponse> =
        ResponseType::new(health_check_response, None);

    Ok(HttpResponse::Ok().json(response))
}
