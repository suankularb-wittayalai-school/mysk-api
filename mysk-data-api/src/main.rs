#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header,
    middleware::{Logger, NormalizePath},
    web::{Data, JsonConfig},
};
use anyhow::{Context as _, Result as AnyhowResult};
use dotenvy::dotenv;
use mysk_lib::{common::config::Config, prelude::*};
use parking_lot::Mutex;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{collections::HashSet, env, time::Duration};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod extractors;
mod routes;

/// The shared state of the application.
pub struct AppState {
    db: PgPool,
    oauth_states: Mutex<HashSet<String>>,
    env: Config,
}

#[actix_web::main]
async fn main() -> AnyhowResult<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::builder().parse(
            #[cfg(debug_assertions)]
            "mysk_data_api=debug,mysk_lib=debug,actix_web=info,sqlx=trace",
            #[cfg(not(debug_assertions))]
            "mysk_data_api=info,mysk_lib=info,actix_web=warn,sqlx=warn",
        )?)
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    dotenv().ok();
    let config = Config::try_from_env()?;
    let Config { host, port, .. } = config;

    tracing::info!(
        "MySK API v{} on {} [{} {}]",
        env!("CARGO_PKG_VERSION"),
        env!("TARGET_TRIPLE"),
        env!("COMMIT_SHORT_HASH"),
        env!("COMMIT_DATE"),
    );
    #[cfg(debug_assertions)]
    tracing::warn!("Running on DEBUG, not optimised for production");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .max_lifetime(Duration::from_secs(1))
        .connect_with(
            config
                .database_url
                .parse::<PgConnectOptions>()
                .context("Failed to parse DATABASE_URL")?
                .ssl_mode(PgSslMode::Require),
        )
        .await
        .context("Failed to connect to the database")?;

    tracing::info!("Established connection to the database successfully");
    tracing::info!("Running on http://{host}:{port}");
    tracing::debug!("You can use this link to login with Google via OAuth:");
    tracing::debug!("{}/auth/oauth/init", config.root_uri);

    let app_state = Data::new(AppState {
        db: pool.clone(),
        oauth_states: Mutex::new(HashSet::new()),
        env: config,
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
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
