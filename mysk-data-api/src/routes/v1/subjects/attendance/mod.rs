use actix_web::web::{scope, ServiceConfig};

pub mod create_report;
pub mod image;
pub mod modify_report;
pub mod query_report_details;
pub mod query_reports;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/image").configure(image::config))
        .service(create_report::create_report)
        .service(modify_report::modify_report)
        .service(query_report_details::query_report_details)
        .service(query_reports::query_reports);
}
