use actix_web::web::ServiceConfig;

pub mod query_classrooms;
pub mod query_classroom_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(query_classrooms::query_classrooms)
        .service(query_classroom_details::query_classroom_details);
}
