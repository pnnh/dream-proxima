use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::{fmt::Display, net::SocketAddr};

use crate::config::is_debug;
use axum::extract::Query;
use axum::response::Html;
use axum::{
    async_trait,
    extract::Extension,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::format::format;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use totp_rs::{Algorithm, TOTP};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::handlers::State;
use crate::models::claims::{AuthBody, AuthPayload, Claims, Keys};
use crate::models::error::AuthError;

// static KEYS: Lazy<Keys> = Lazy::new(|| {
//     let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     Keys::new(secret.as_bytes())
// });

#[derive(Deserialize)]
pub struct RegisterQuery {
    account: Option<String>,
}
pub async fn register_handler(
    Query(args): Query<RegisterQuery>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Html<String>, AuthError> {
    // 仅在开发环境下可以访问
    if !is_debug() {
        return Err(AuthError::MissingCredentials);
    }
    let mut account: String = args.account.unwrap_or("".to_string());
    if account.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let secret = &state.config.totp_secret;
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
        Some("dream".to_string()),
        account,
    )
    .unwrap();
    let url = totp.get_url();
    println!("{}", url);
    let code = totp.get_qr().unwrap();
    println!("{}", code);

    let page_data = &json!({
        "totp_url": url,
        "totp_qrcode": code,
    });

    let result = state
        .registry
        .render("account_register", page_data)
        .map_err(|err| AuthError::MissingCredentials)?;

    Ok(Html(result))
}

pub async fn login_handler(
    Json(payload): Json<AuthPayload>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.account.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let secret = &state.config.totp_secret;
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        &secret,
        Some("dream".to_string()),
        payload.account.clone(),
    )
    .unwrap();

    let ok = totp
        .check_current(payload.code.as_str())
        .map_err(|_| AuthError::WrongCredentials)?;
    if !ok {
        return Err(AuthError::WrongCredentials);
    }

    let conn = state
        .pool
        .get()
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    let uname = &payload.account;
    let query_result = conn
        .query("select accounts.pk from accounts where uname=$1", &[&uname])
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    if query_result.len() < 1 {
        return Err(AuthError::WrongCredentials);
    }

    let pk: String = query_result[0].get("pk");

    let claims = Claims {
        exp: 2000000000, // May 2033
        user: pk,
    };
    let jwt_keys = Keys::new(&state.config.jwt_secret.as_bytes());
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &jwt_keys.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
