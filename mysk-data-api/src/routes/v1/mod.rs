use actix_web::web;

pub(crate) mod students;

pub fn config(cfg: &mut web::ServiceConfig) {
    students::config(cfg);
}
