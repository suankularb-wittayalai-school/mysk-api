use actix_web::web;

pub mod query_electives;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(query_electives::query_elective_subject);
}
