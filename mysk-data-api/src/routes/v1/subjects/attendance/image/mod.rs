use actix_web::web::ServiceConfig;

pub mod get_report_image;
pub mod upload_report_image;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_report_image::get_report_image);
    cfg.service(upload_report_image::upload_report_image);
}
