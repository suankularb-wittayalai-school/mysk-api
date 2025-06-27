use actix_web::web::{ServiceConfig, scope};

pub mod rsvp;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/rsvp").configure(rsvp::config));
}
