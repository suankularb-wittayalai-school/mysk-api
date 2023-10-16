use actix_web::web;

pub mod get_student_by_id;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_student_by_id::get_student_by_id);
}
