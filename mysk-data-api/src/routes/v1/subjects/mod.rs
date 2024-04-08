use actix_web::web;

pub mod electives;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/electives").configure(electives::config));
}
