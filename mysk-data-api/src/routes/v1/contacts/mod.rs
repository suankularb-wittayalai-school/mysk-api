use actix_web::web::ServiceConfig;

pub mod delete_contacts;
pub mod modify_contacts;
pub mod query_contact_details;
pub mod query_contacts;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(delete_contacts::delete_contacts)
        .service(modify_contacts::modify_contacts)
        .service(query_contact_details::query_contact_details)
        .service(query_contacts::query_contacts);
}
