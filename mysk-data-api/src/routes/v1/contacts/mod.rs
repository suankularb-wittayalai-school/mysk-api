use actix_web::web::ServiceConfig;

pub mod delete_contacts;
pub mod modify_contact;
pub mod query_contact_details;
pub mod query_contacts;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(delete_contacts::delete_contacts)
        .service(modify_contact::modify_contact)
        .service(query_contact_details::query_contact_details)
        .service(query_contacts::query_contacts);
}
