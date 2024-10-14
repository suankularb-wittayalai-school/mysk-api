use actix_web::web::ServiceConfig;

pub mod create_teacher_contacts;
pub mod get_teacher_by_id;
pub mod modify_teacher;
pub mod query_teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_teacher_by_id::get_teacher_by_id);
    cfg.service(query_teachers::query_teachers);
    cfg.service(create_teacher_contacts::create_teacher_contacts);
    cfg.service(modify_teacher::modify_teacher);
}
