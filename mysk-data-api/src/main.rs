#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Logger, NormalizePath},
    web::{Data, JsonConfig},
    App, HttpServer,
};
use dotenv::dotenv;
use log::{debug, error, info, warn};
use mysk_lib::{common::config::Config, prelude::*};
use parking_lot::Mutex;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{collections::HashSet, env, io, process};

mod extractors;
mod routes;

/// The shared state of the application.
pub struct AppState {
    db: PgPool,
    oauth_states: Mutex<HashSet<String>>,
    env: Config,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // TODO: Change to tracing instead, the `unsafe` blocks are temporary due to changes in Rust 2024 edition's API
    if env::var_os("RUST_LOG").is_none() {
        #[cfg(debug_assertions)]
        unsafe { env::set_var("RUST_LOG", "mysk_data_api=debug,actix_web=debug,sqlx=debug"); }
        #[cfg(not(debug_assertions))]
        unsafe { env::set_var("RUST_LOG", "mysk_data_api=info,actix_web=info"); }
    }
    env_logger::init();
    let config = Config::init();
    let host = config.host;
    let port = config.port;

    info!(
        "MySK API v{} on {} [{} {}]",
        env!("CARGO_PKG_VERSION"),
        env!("TARGET_TRIPLE"),
        env!("COMMIT_SHORT_HASH"),
        env!("COMMIT_DATE"),
    );
    #[cfg(debug_assertions)]
    warn!("Running on DEBUG, not optimised for production");

    let pool = PgPoolOptions::new()
        .max_connections(15)
        .connect_with(
            config
                .database_url
                .parse::<PgConnectOptions>()
                .unwrap()
                .ssl_mode(PgSslMode::Require),
        )
        .await
        .map_err(|err| {
            error!("Failed to connect to the database: {err:?}");
            process::exit(1);
        })
        .unwrap();

    info!("Established connection to the database successfully");
    info!("Running on http://{host}:{port}");
    debug!("You can use this link to login with Google via OAuth:");
    debug!("{}/auth/oauth/init", config.root_uri);

    let app_state = Data::new(AppState {
        db: pool.clone(),
        oauth_states: Mutex::new(HashSet::new()),
        env: config.clone(),
    });

    HttpServer::new(move || {
        let cors_middleware = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_origin("https://mysk.school")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::ACCEPT,
                header::HeaderName::from_lowercase(b"x-api-key").unwrap(),
            ])
            .supports_credentials();

        App::new()
            .app_data(app_state.clone())
            .app_data(JsonConfig::default().error_handler(|err, req| {
                Error::InvalidRequest(format!("{err}"), req.path().into()).into()
            }))
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(cors_middleware)
            .configure(routes::config)
    })
    .bind((host, port))
    .map_err(|_| panic!("Unable to bind to address {host}:{port}! Perhaps it is in use?"))
    .unwrap()
    .run()
    .await
}
