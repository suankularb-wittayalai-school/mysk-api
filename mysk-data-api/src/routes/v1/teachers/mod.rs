use actix_web::web::ServiceConfig;

pub mod create_teacher_contacts;
pub mod modify_teacher;
pub mod query_teacher_details;
pub mod query_teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_teacher_contacts::create_teacher_contacts);
    cfg.service(modify_teacher::modify_teacher);
    cfg.service(query_teacher_details::query_teacher_details);
    cfg.service(query_teachers::query_teachers);
}
