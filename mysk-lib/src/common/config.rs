use anyhow::{Context as _, Result as AnyhowResult};
use std::env::var;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub host: IpAddr,
    pub port: u16,
    pub root_uri: String,
    pub supabase_secret_key: String,
    pub supabase_uri: String,
    pub token_max_age: u64,
    pub token_secret: String,
}

impl Config {
    pub fn try_from_env() -> AnyhowResult<Config> {
        let database_url = var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let google_oauth_client_id =
            var("GOOGLE_OAUTH_CLIENT_ID").context("GOOGLE_OAUTH_CLIENT_ID must be set")?;
        let google_oauth_client_secret =
            var("GOOGLE_OAUTH_CLIENT_SECRET").context("GOOGLE_OAUTH_CLIENT_SECRET must be set")?;
        let host = var("HOST")
            .unwrap_or("0.0.0.0".into())
            .parse()
            .context("HOST must be a valid IP address")?;
        let port = var("PORT")
            .unwrap_or("8000".into())
            .parse()
            .context("PORT must be a valid port number")?;
        let root_uri = var("ROOT_URI").context("ROOT_URI must be set")?;
        let supabase_secret_key =
            var("SUPABASE_SECRET_KEY").context("SUPABASE_SECRET_KEY must be set")?;
        let supabase_uri = var("SUPABASE_URI").context("SUPABASE_URI must be set")?;
        let token_max_age = var("TOKEN_MAXAGE")
            .context("TOKEN_MAXAGE must be set")?
            .parse()
            .context("TOKEN_MAXAGE must be a positive integer")?;
        let token_secret = var("TOKEN_SECRET").context("TOKEN_SECRET must be set")?;

        Ok(Config {
            database_url,
            google_oauth_client_id,
            google_oauth_client_secret,
            host,
            port,
            root_uri,
            supabase_secret_key,
            supabase_uri,
            token_max_age,
            token_secret,
        })
    }
}
