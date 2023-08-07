use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

mod routes;

pub struct AppState {
    db: Pool<Postgres>,
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // builder
    //     .set_private_key_file("ssl/privkey.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("ssl/cert.pem").unwrap();

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
                // Custom headers
                header::HeaderName::from_lowercase(b"x-api-key").unwrap(),
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
            }))
            // .service(web::scope("/api/v1").configure(routes::config))
            .configure(routes::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    // .bind_openssl(("0.0.0.0", 4430), builder)?
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
