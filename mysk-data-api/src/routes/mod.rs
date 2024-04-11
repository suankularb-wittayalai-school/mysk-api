use actix_web::web::{scope, ServiceConfig};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub(crate) mod auth;
mod doc;
pub(crate) mod health;
pub(crate) mod test;
pub(crate) mod v1;

use doc::ApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/v1").configure(v1::config));
    cfg.service(scope("/auth").configure(auth::config));
    cfg.service(health::health_check);
    cfg.service(test::test);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
