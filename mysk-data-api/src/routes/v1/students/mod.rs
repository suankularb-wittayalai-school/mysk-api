use actix_web::web::ServiceConfig;

pub mod create_student_contacts;
pub mod modify_student;
pub mod query_student_details;
pub mod query_students;
pub mod query_students_cheer_practice_attendance;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_student_contacts::create_student_contacts)
        .service(modify_student::modify_student)
        .service(query_student_details::query_student_details)
        .service(query_students::query_students)
        .service(
            query_students_cheer_practice_attendance::query_students_cheer_practice_attendances,
        );
}
