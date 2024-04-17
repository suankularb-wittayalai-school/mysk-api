use actix_web::web::ServiceConfig;

pub mod get_student_by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_student_by_id::get_student_by_id);
}
