use crate::{extractors::ExtractorFuture, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use futures::{future, FutureExt as _};
use mysk_lib::{
    auth::key::{ApiKey, PrefixedApiKey},
    prelude::*,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::query_as;

/// Extractor to allow only clients with a valid API key.
#[derive(Serialize)]
pub struct ApiKeyHeader(ApiKey);

impl FromRequest for ApiKeyHeader {
    type Error = Error;
    type Future = ExtractorFuture<Self>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let app_state = req
            .app_data::<Data<AppState>>()
            .expect("Irrecoverable error, AppState is None");
        let pool = app_state.db.clone();
        let source = req.path().to_string();
        let token = if let Some(token) = req.headers().get("X-Api-Key") {
            let Ok(token) = token.to_str() else {
                return future::err(Error::InvalidApiKey("Invalid API Key".to_string(), source))
                    .boxed();
            };

            PrefixedApiKey::try_from(token.to_string())
        } else {
            return future::err(Error::MissingApiKey("Missing API Key".to_string(), source))
                .boxed();
        };

        async move {
            let token = token?;
            let mut hasher = Sha256::new();
            hasher.update(token.get_long_token().as_bytes());
            let hash = bs58::encode(hasher.finalize()).into_string();

            let api_key = query_as!(
                ApiKey,
                "\
                SELECT * FROM user_api_keys \
                WHERE long_token_hash = $1 AND short_token = $2 \
                AND (expire_at > NOW() OR expire_at IS NULL)\
                ",
                hash,
                token.get_short_token(),
            )
            .fetch_one(&pool)
            .await?;

            Ok(ApiKeyHeader(api_key))
        }
        .boxed()
    }
}
