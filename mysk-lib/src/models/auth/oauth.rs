use jsonwebtoken::Validation;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::models::common::config::Config;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

impl GoogleUserResult {
    pub fn from_token_payload(payload: TokenPayload) -> Self {
        Self {
            id: payload.sub,
            email: payload.email,
            verified_email: payload.email_verified,
            name: payload.name,
            given_name: payload.given_name,
            family_name: payload.family_name,
            picture: payload.picture,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenPayload {
    // Add fields here as needed to capture the claims from the ID token
    // For example: iss, aud, exp, sub, email, etc.
    #[serde(rename = "aud")]
    _aud: String,
    #[serde(rename = "azp")]
    _azp: String,
    email: String,
    email_verified: bool,
    #[serde(rename = "exp")]
    _exp: usize,
    given_name: String,
    family_name: String,
    #[serde(rename = "iat")]
    _iat: usize,
    #[serde(rename = "iss")]
    _iss: String,
    #[serde(rename = "jti")]
    _jti: String,
    name: String,
    #[serde(rename = "nbf")]
    _nbf: usize,
    picture: String,
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePublicKey {
    alg: String,
    e: String,
    kid: String,
    kty: String,
    n: String,
    #[serde(rename = "use")]
    use_: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePublicKeys {
    keys: Vec<GooglePublicKey>,
}

pub async fn verify_id_token(id_token: &str, env: &Config) -> Result<TokenPayload> {
    let public_keys_url = "https://www.googleapis.com/oauth2/v3/certs";
    let public_keys_response = Client::new().get(public_keys_url).send().await;

    let public_keys_response = match public_keys_response {
        Ok(response) => response,
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "verify_id_token".to_string(),
            ))
        }
    };

    if !public_keys_response.status().is_success() {
        return Err(Error::InternalSeverError(
            "Failed to get public keys".to_string(),
            "verify_id_token".to_string(),
        ));
    }

    // public key response is array of keys convert to hashmap with kid as key
    let public_keys = public_keys_response.json().await;

    let public_keys: GooglePublicKeys = match public_keys {
        Ok(keys) => keys,
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "verify_id_token".to_string(),
            ))
        }
    };

    let public_keys: HashMap<String, String> = public_keys.keys.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<String, String>, key| {
            acc.insert(key.kid, key.n);
            acc
        },
    );

    // dbg!(&public_keys);

    let header = jsonwebtoken::decode_header(id_token);

    let header = match header {
        Ok(header) => header,
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "verify_id_token".to_string(),
            ))
        }
    };

    let kid = match header.kid {
        Some(kid) => kid,
        None => {
            return Err(Error::InternalSeverError(
                "No kid in header".to_string(),
                "verify_id_token".to_string(),
            ))
        }
    };

    let public_key = public_keys[kid.as_str()].as_str();

    let public_key = jsonwebtoken::DecodingKey::from_rsa_components(public_key, "AQAB");

    let public_key = match public_key {
        Ok(key) => key,
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "verify_id_token".to_string(),
            ))
        }
    };

    let mut validation = Validation::new(header.alg);

    validation.set_audience(&[env.google_oauth_client_id.to_owned()]);
    validation.iss = Some(HashSet::from(["https://accounts.google.com".to_owned()]));

    // dbg!(&validation);

    let token_payload = jsonwebtoken::decode::<TokenPayload>(id_token, &public_key, &validation);
    let token_payload = match token_payload {
        Ok(payload) => payload,
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "verify_id_token".to_string(),
            ))
        }
    };

    Ok(token_payload.claims)
}
