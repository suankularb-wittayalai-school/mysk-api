use crate::{common::config::Config, prelude::*};
use rand::{TryRngCore as _, rngs::OsRng};
use reqwest::{
    Client,
    header::{CONTENT_LENGTH, HeaderValue},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mta: Option<Uuid>,
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
    name: String,
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

#[derive(Debug, Serialize)]
struct GoogleOAuthInitQueryParams<'a> {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    access_type: String,
    state: &'a str,
    include_granted_scopes: bool,
    hd: String,
    prompt: Option<String>,
}

pub fn generate_oauth_init_url(client_id: &str, redirect_uri: &str) -> Result<(String, String)> {
    let mut state = [0u8; 32];
    OsRng.try_fill_bytes(&mut state).unwrap();
    let state = format!("{:x}", Sha256::new().chain_update(state).finalize());
    let query_params = GoogleOAuthInitQueryParams {
        client_id: client_id.to_string(),
        redirect_uri: redirect_uri.to_string(),
        response_type: "code".to_string(),
        scope: [
            "openid",
            "https://www.googleapis.com/auth/userinfo.email",
            "https://www.googleapis.com/auth/userinfo.profile",
        ]
        .join(" ")
        .to_string(),
        access_type: "online".to_string(),
        state: &state,
        include_granted_scopes: true,
        hd: "sk.ac.th".to_string(),
        #[cfg(debug_assertions)]
        prompt: Some("select_account".to_string()),
        #[cfg(not(debug_assertions))]
        prompt: None,
    };

    Ok((
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?{}",
            serde_qs::to_string(&query_params)?,
        ),
        state,
    ))
}

#[derive(Debug, Serialize)]
struct CodeExchangeQueryParams {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

#[derive(Debug, Deserialize)]
struct CodeExchangeResponse {
    id_token: String,
}

pub async fn exchange_oauth_code(
    code: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> Result<String> {
    let query_params = CodeExchangeQueryParams {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        code: code.to_string(),
        grant_type: "authorization_code".to_string(),
        redirect_uri: redirect_uri.to_string(),
    };
    let code_exchange_response = Client::new()
        .post(format!(
            "https://oauth2.googleapis.com/token?{}",
            serde_qs::to_string(&query_params)?,
        ))
        .header(CONTENT_LENGTH, HeaderValue::from_static("0"))
        .send()
        .await?;

    Ok(code_exchange_response
        .json::<CodeExchangeResponse>()
        .await?
        .id_token)
}

pub async fn verify_id_token(id_token: &str, env: &Config) -> Result<TokenPayload> {
    let public_keys_url = "https://www.googleapis.com/oauth2/v3/certs";
    let public_keys_response = Client::new()
        .get(public_keys_url)
        .send()
        .await?
        .error_for_status()?;
    // if !public_keys_response.status().is_success() {
    //     return Err(Error::InternalServerError(
    //         "Failed to fetch public keys from googleapis".to_string(),
    //         "verify_id_token".to_string(),
    //     ));
    // }

    // public key response is array of keys convert to hashmap with kid as key
    let public_keys = public_keys_response.json::<GooglePublicKeys>().await?;
    let public_keys: HashMap<String, String> = public_keys.keys.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<String, String>, key| {
            acc.insert(key.kid, key.n);
            acc
        },
    );

    let header = jsonwebtoken::decode_header(id_token)?;
    let Some(kid) = header.kid else {
        return Err(Error::InternalServerError(
            "No `kid` field in header".to_string(),
            "verify_id_token".to_string(),
        ));
    };

    let public_key = jsonwebtoken::DecodingKey::from_rsa_components(&public_keys[&kid], "AQAB")?;

    let mut validation = jsonwebtoken::Validation::new(header.alg);
    validation.set_audience(&[env.google_oauth_client_id.as_str()]);
    validation.iss = Some(HashSet::from(["https://accounts.google.com".to_owned()]));

    let token_payload = jsonwebtoken::decode::<TokenPayload>(id_token, &public_key, &validation)?;

    Ok(token_payload.claims)
}
