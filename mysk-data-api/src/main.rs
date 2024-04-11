use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use mysk_lib::models::common::config::Config;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, io, process};

mod middlewares;
mod routes;

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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let env = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
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
                env: env.clone(),
            }))
            .configure(routes::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
