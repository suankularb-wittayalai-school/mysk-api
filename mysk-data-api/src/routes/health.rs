use actix_web::{get, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::AppState;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    server_time: String,
    database_connection: bool,
    database_response_time: String,
}

impl HealthCheckResponse {
    pub fn new(database_connection: bool, database_response_time: String) -> Self {
        // let (total_memory, used_memory) = get_memory_usage();

        HealthCheckResponse {
            server_time: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            database_connection,
            database_response_time,
            // memory_consumption: used_memory as f32 / 1024.0 / 1024.0,
        }
    }
}

#[get("/health-check")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.db;
    let database_connection = pool.acquire().await.is_ok();

    // calculate database response time
    let start = std::time::Instant::now();

    let _ = sqlx::query("SELECT 1").execute(pool).await.is_ok();

    let database_response_time = start.elapsed().as_millis().to_string();

    let health_check_response =
        HealthCheckResponse::new(database_connection, database_response_time);
    // let response: ResponseType<HealthCheckResponse, _> =
    //     common::ResponseType::new(health_check_response, None::<ErrorType<String>>, None);

    let response = health_check_response;

    HttpResponse::Ok().json(response)
}
