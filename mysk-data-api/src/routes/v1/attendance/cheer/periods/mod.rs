use actix_web::web::ServiceConfig;

pub mod check_practice_attendance;
pub mod query_practice_period;
pub mod query_practice_period_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(check_practice_attendance::check_practice_attendance)
        .service(query_practice_period::query_practice_period)
        .service(query_practice_period_details::query_practice_period_details);
}
