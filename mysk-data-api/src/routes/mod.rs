use actix_web::web;

pub(crate) mod health;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check);
    // cfg.service(
    //     SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    // );
}
