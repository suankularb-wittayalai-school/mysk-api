use crate::{routes::auth::gsi_login::GoogleTokenResponse, AppState};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get,
    web::{Data, Query, Redirect},
    HttpResponse, Responder,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mysk_lib::{
    auth::oauth::{
        exchange_oauth_code, generate_oauth_init_url, verify_id_token, GoogleUserResult,
        TokenClaims,
    },
    common::response::ResponseType,
    models::user::User,
    prelude::*,
};
use serde::Deserialize;

#[get("/oauth/init")]
pub async fn oauth_initiator(data: Data<AppState>) -> Result<impl Responder> {
    let (redirect_url, state) = generate_oauth_init_url(
        &data.env.google_oauth_client_id,
        format!("{}/auth/oauth/google", &data.env.root_uri).as_str(),
    );

    {
        let mut guard = data.oauth_states.lock();
        let oauth_states = &mut *guard;
        oauth_states.insert(state.clone());
    }

    Ok(Redirect::to(redirect_url))
}

#[derive(Deserialize)]
struct GoogleOAuthCodeRequest {
    code: String,
    state: String,
}

#[allow(clippy::cast_possible_wrap)]
#[get("/oauth/google")]
pub async fn google_oauth_handler(
    data: Data<AppState>,
    request_query: Query<GoogleOAuthCodeRequest>,
) -> Result<impl Responder> {
    let code = &request_query.code;
    let state = &request_query.state;

    {
        let mut guard = data.oauth_states.lock();
        let oauth_states = &mut *guard;

        if !oauth_states.contains(state) {
            return Err(Error::InvalidToken(
                "OAuth state mismatch".to_string(),
                "/auth/oauth/google".to_string(),
            ));
        }

        oauth_states.remove(state);
    }

    let id_token = exchange_oauth_code(
        code,
        &data.env.google_oauth_client_id,
        &data.env.google_oauth_client_secret,
        format!("{}/auth/oauth/google", &data.env.root_uri).as_str(),
    )
    .await?;

    let google_id_data = match verify_id_token(&id_token, &data.env).await {
        Ok(data) => data,
        Err(err) => {
            return Err(Error::InvalidToken(
                err.to_string(),
                "/auth/oauth/google".to_string(),
            ));
        }
    };

    let google_user = GoogleUserResult::from_token_payload(google_id_data);
    let user_id = match User::get_by_email(&data.db, &google_user.email).await {
        Ok(Some(user)) => user.id,
        Ok(None) => {
            return Err(Error::EntityNotFound(
                "User not found".to_string(),
                "/auth/oauth/google".to_string(),
            ))
        }
        Err(err) => {
            return Err(Error::InternalSeverError(
                err.to_string(),
                "/auth/oauth/google".to_string(),
            ))
        }
    };

    let jwt_secret = data.env.token_secret.clone();
    let now = Utc::now();
    let iat = usize::try_from(now.timestamp()).unwrap();
    let exp = usize::try_from((now + Duration::minutes(data.env.token_max_age as i64)).timestamp())
        .unwrap();
    let claims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    );

    match token {
        Ok(token) => {
            let cookie = Cookie::build("token", token.clone())
                .secure(true)
                .http_only(true)
                .max_age(ActixWebDuration::days(30))
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish();

            let response: ResponseType<GoogleTokenResponse> = ResponseType::new(
                GoogleTokenResponse {
                    access_token: token,
                    expires_in: data.env.token_max_age * 60,
                    token_type: "Bearer".to_string(),
                    scope: "openid email profile".to_string(),
                    id_token,
                },
                None,
            );

            Ok(HttpResponse::Ok().cookie(cookie).json(response))
        }
        Err(err) => Err(Error::InternalSeverError(
            err.to_string(),
            "/auth/oauth/google".to_string(),
        )),
    }
}
