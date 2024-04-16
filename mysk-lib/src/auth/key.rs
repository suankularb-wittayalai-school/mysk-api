use crate::prelude::*;
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, RngCore};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub long_token_hash: String,
    pub short_token: String,
    pub expire_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct PrefixedApiKey {
    prefix: String,
    short_token: String,
    long_token: String,
}

fn generate_api_key(length: usize) -> String {
    let mut key = [0u8; 64];
    OsRng.fill_bytes(&mut key);
    let key = &key[..length];
    let bytes = key.to_vec();

    bs58::encode(&bytes).into_string()
}

impl ApiKey {
    #[allow(clippy::cast_precision_loss)]
    pub async fn create(pool: &PgPool, user_id: Uuid, expire_days: Option<i64>) -> Result<String> {
        // Generate a new API key
        let short_token = generate_api_key(8);
        let long_token = generate_api_key(24);

        // Hash the API key with SHA256
        let mut hasher = Sha256::new();
        hasher.update(long_token.as_bytes());
        let hash = bs58::encode(hasher.finalize()).into_string();

        let res = sqlx::query!(
            r#"
            INSERT INTO user_api_keys (user_id, short_token, long_token_hash, expire_at)
            VALUES ($1, $2, $3, NOW() + ($4 * INTERVAL '1 DAY'))
            "#,
            user_id,
            short_token,
            hash,
            match expire_days {
                Some(days) => Some(days as f64),
                None => None,
            },
        )
        .execute(pool)
        .await;

        match res {
            Ok(_) => Ok(format!("mysk_{short_token}_{long_token}")),
            Err(err) => Err(Error::InternalSeverError(
                err.to_string(),
                "ApiKey Model".to_string(),
            )),
        }
    }
}

impl PrefixedApiKey {
    pub fn new(prefix: String, short_token: String, long_token: String) -> Self {
        Self {
            prefix,
            short_token,
            long_token,
        }
    }

    pub fn get_short_token(&self) -> &str {
        &self.short_token
    }

    pub fn get_long_token(&self) -> &str {
        &self.long_token
    }

    #[must_use]
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
}

impl From<String> for PrefixedApiKey {
    fn from(api_key: String) -> Self {
        let tokens: Vec<&str> = api_key.split('_').collect();
        let prefix = tokens[0].to_string();
        let short_token = tokens[1].to_string();
        let long_token = tokens[2].to_string();

        Self {
            prefix,
            short_token,
            long_token,
        }
    }
}
impl ToString for PrefixedApiKey {
    fn to_string(&self) -> String {
        format!("{}_{}_{}", self.prefix, self.short_token, self.long_token)
    }
}
