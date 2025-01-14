use actix_web::web::ServiceConfig;

pub mod create_student_contacts;
pub mod modify_student;
pub mod query_student_details;
pub mod query_students;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_student_contacts::create_student_contacts);
    cfg.service(modify_student::modify_student);
    cfg.service(query_student_details::query_student_details);
    cfg.service(query_students::query_students);
}
