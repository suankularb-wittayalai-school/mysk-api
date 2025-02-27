use actix_web::web::{scope, ServiceConfig};

pub mod rsvp;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/rsvp").configure(rsvp::config));
}
