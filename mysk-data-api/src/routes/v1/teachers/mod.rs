use actix_web::web;

pub mod get_teacher_by_id;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_teacher_by_id::get_teacher_by_id);
}
