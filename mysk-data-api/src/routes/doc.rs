use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut();

        let components = match components {
            Some(components) => components,
            None => return,
        };

        components.add_security_scheme(
            "JWT Token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("Bearer")
                    .build(),
            ),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "MySK Data API",
        description = "API to interact with school data"
    ),
    // components(schemas()),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
