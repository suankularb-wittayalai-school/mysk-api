#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Logger, NormalizePath},
    web::Data,
    App, HttpServer,
};
use dotenv::dotenv;
use mysk_lib::common::config::Config;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, io, process};

mod extractors;
mod routes;

/// The shared state of the application.
pub struct AppState {
    db: PgPool,
    env: Config,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
    let config = Config::init();
    let host = config.host;
    let port = config.port;

    let pool = match PgPoolOptions::new()
        .max_connections(15)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {err:?}");
            process::exit(1);
        }
    };

    println!("🚀 MySK API Server started successfully");

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
            .app_data(Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .configure(routes::config)
            .wrap(cors_middleware)
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
    })
    .bind((host, port))
    .map_err(|_| panic!("Unable to bind to address {host}:{port}! Perhaps it is in use?"))
    .unwrap()
    .run()
    .await
}
