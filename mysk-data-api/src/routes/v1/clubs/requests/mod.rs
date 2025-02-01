use actix_web::web::ServiceConfig;

pub mod delete_club_requests;
pub mod query_club_request_details;
pub mod query_club_requests;
pub mod update_club_requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(delete_club_requests::delete_club_requests)
        .service(query_club_request_details::query_club_request_details)
        .service(query_club_requests::query_club_requests)
        .service(update_club_requests::update_club_requests);
}
