use actix_web::web::ServiceConfig;

pub mod query_practice_period_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(query_practice_period_details::query_practice_period_details);
}
