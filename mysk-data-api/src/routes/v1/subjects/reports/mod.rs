use actix_web::web::ServiceConfig;

pub mod query_report_details;
pub mod query_reports;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(query_report_details::query_report_details);
    cfg.service(query_reports::query_reports);
}
