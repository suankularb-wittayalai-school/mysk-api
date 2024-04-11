use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub client_origin: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
}

impl Config {
    pub fn init() -> Config {
        let client_origin = var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = var("TOKEN_EXPIRED_IN").expect("TOKEN_EXPIRED_IN must be set");
        let jwt_max_age = var("TOKEN_MAXAGE").expect("TOKEN_MAXAGE must be set");
        let google_oauth_client_id =
            var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret =
            var("GOOGLE_OAUTH_CLIENT_SECRET").expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        Config {
            client_origin,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            google_oauth_client_id,
            google_oauth_client_secret,
        }
    }
}
