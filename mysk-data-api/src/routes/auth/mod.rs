pub(crate) mod google_login;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(google_login::google_oauth_handler);
}
