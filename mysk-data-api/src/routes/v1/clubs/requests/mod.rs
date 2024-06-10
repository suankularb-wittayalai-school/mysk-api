use actix_web::web::ServiceConfig;

pub mod delete_club_requests;
pub mod query_club_request_details;
pub mod query_club_requests;
pub mod update_club_requests;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(delete_club_requests::delete_club_requests);
    cfg.service(query_club_request_details::query_club_request_details);
    cfg.service(query_club_requests::query_club_requests);
    cfg.service(update_club_requests::update_club_requests);
}
