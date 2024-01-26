use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{prelude::FromRow, Error as SqlxError, PgPool};
use std::io::{Error, ErrorKind};
use uuid::Uuid;

use rand::rngs::OsRng;
use rand::RngCore; // Secure CSPRNG

// use prefixed_api_key::PrefixedApiKeyController;

#[derive(Debug, Serialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub user_ud: Uuid,
    pub hash: String,
    pub expire_at: Option<DateTime<Utc>>,
}

fn generate_api_key(length: usize) -> String {
    let mut key = [0u8; 64];
    OsRng.fill_bytes(&mut key);
    let key = &key[..length];
    let bytes = key.to_vec();

    bs58::encode(&bytes).into_string() // Encode bytes to Base58 string
}

impl ApiKey {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        expire_days: i64,
    ) -> Result<String, SqlxError> {
        // Generate a new API key
        let short_token = generate_api_key(8);
        let long_token = generate_api_key(24);

        // Hash the API key with SHA256
        let mut hasher = Sha256::new();
        hasher.update(long_token.as_bytes());
        let hash = hasher.finalize();
        // make sure the hash is a valid UTF-8 string
        let hash = String::from_utf8(hash.to_vec()).unwrap();

        // Insert the API key into the database
        sqlx::query!(
            r#"
            INSERT INTO user_api_keys (user_id, short_token, hash, expire_at)
            VALUES ($1, $2, $3, NOW() + ($4 * INTERVAL '1 DAY'))
            "#,
            user_id,
            short_token,
            hash,
            expire_days as f32
        )
        .execute(pool)
        .await?;

        Ok(format!("mysk_{}_{}", short_token, long_token))
    }
}
