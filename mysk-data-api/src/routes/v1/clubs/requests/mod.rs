use actix_web::web::ServiceConfig;

pub mod delete_club_request;
pub mod query_club_request_details;
pub mod query_club_requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(delete_club_request::delete_club_request)
        .service(query_club_request_details::query_club_request_details)
        .service(query_club_requests::query_club_requests);
}
