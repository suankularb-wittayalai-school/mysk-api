use std::env::var;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub client_origin: String,
    pub database_url: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub host: IpAddr,
    pub port: u16,
    pub token_expired_in: String,
    pub token_max_age: u64,
    pub token_secret: String,
}

impl Config {
    pub fn init() -> Config {
        let client_origin = var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
        let google_oauth_client_id =
            var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret =
            var("GOOGLE_OAUTH_CLIENT_SECRET").expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let host = var("HOST")
            .unwrap_or("0.0.0.0".into())
            .parse::<IpAddr>()
            .expect("HOST must be a valid IP address");
        let port = var("PORT")
            .unwrap_or("8000".into())
            .parse::<u16>()
            .expect("PORT must be a valid port number");
        let token_expired_in = var("TOKEN_EXPIRED_IN").expect("TOKEN_EXPIRED_IN must be set");
        let token_max_age = var("TOKEN_MAXAGE")
            .expect("TOKEN_MAXAGE must be set")
            .parse::<u64>()
            .expect("TOKEN_MAXAGE must be a positive integer");
        let token_secret = var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");

        Config {
            client_origin,
            database_url,
            google_oauth_client_id,
            google_oauth_client_secret,
            host,
            port,
            token_expired_in,
            token_max_age,
            token_secret,
        }
    }
}
