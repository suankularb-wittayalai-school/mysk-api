use actix_web::web::{ServiceConfig, scope};

pub mod rsvp;
pub mod query_certificate_details;
pub mod query_certificates;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/rsvp").configure(rsvp::config))
    .service(query_certificate_details::query_certificate_details)
    .service(query_certificates::query_certificates);
}
