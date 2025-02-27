use actix_web::web::ServiceConfig;

pub mod in_rsvp_period;
pub mod modify_invitation;
pub mod query_invitation_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(in_rsvp_period::in_rsvp_period)
        .service(modify_invitation::modify_invitation)
        .service(query_invitation_details::query_invitation_details);
}
