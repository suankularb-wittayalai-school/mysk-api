use actix_web::web::{PayloadConfig, ServiceConfig};

pub mod get_report_image;
pub mod modify_report_image;
pub mod upload_report_image;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_report_image::get_report_image)
        // 4.5 * 2**20 = ~4.5 MiB (Vercel serverless functions limit)
        .app_data(PayloadConfig::new(4_718_592))
        .service(modify_report_image::modify_report_image)
        .service(upload_report_image::upload_report_image);
}
