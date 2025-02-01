use actix_web::web::ServiceConfig;

pub mod create_teacher_contacts;
pub mod modify_teacher;
pub mod query_teacher_details;
pub mod query_teachers;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_teacher_contacts::create_teacher_contacts)
        .service(modify_teacher::modify_teacher)
        .service(query_teacher_details::query_teacher_details)
        .service(query_teachers::query_teachers);
}
