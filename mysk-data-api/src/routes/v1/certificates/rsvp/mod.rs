use actix_web::web::ServiceConfig;

pub mod in_rsvp_period;
pub mod modify_invitation;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(in_rsvp_period::in_rsvp_period)
        .service(modify_invitation::modify_invitation);
}
