use actix_web::{get, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use mysk_lib::models::common::response::ResponseType;

use crate::AppState;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    server_time: String,
    database_connection: bool,
    database_response_time: u128,
}

impl HealthCheckResponse {
    pub async fn new(pool: &sqlx::PgPool) -> Self {
        let start = std::time::Instant::now();

        let database_connection = sqlx::query("SELECT 1").execute(pool).await.is_ok();

        let database_response_time = start.elapsed().as_millis();

        HealthCheckResponse {
            server_time: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            database_connection,
            database_response_time,
            // memory_consumption: used_memory as f32 / 1024.0 / 1024.0,
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
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.db;

    let health_check_response = HealthCheckResponse::new(pool).await;
    let response: ResponseType<HealthCheckResponse> =
        ResponseType::new(health_check_response, None);

    HttpResponse::Ok().json(response)
}
