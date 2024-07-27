use actix_web::web::ServiceConfig;

pub mod get_teacher_by_id;
pub mod query_teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_teacher_by_id::get_teacher_by_id);
    cfg.service(query_teachers::query_teachers);
}
