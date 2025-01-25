use actix_web::web::ServiceConfig;

pub mod create_class_report;
pub mod modify_class_report;
pub mod query_report_details;
pub mod query_reports;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_class_report::create_class_report);
    cfg.service(modify_class_report::modify_class_report);
    cfg.service(query_report_details::query_report_details);
    cfg.service(query_reports::query_reports);
}
