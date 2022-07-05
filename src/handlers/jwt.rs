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
    Extension(state): Extension<Arc<State<'_>>>,
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
    Extension(state): Extension<Arc<State<'_>>>,
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
        payload.account,
    )
    .unwrap();
    // let token = totp.generate_current().unwrap();
    // println!("{}", token);

    // if let Err(_) = totp.check_current(payload.token.as_str()) {
    //     return Err(AuthError::WrongCredentials);
    // }
    let ok = totp
        .check_current(payload.code.as_str())
        .map_err(|_| AuthError::WrongCredentials)?;
    if !ok {
        return Err(AuthError::WrongCredentials);
    }
    // Here you can check the user credentials from a database
    // if payload.client_id != "foo" || payload.client_secret != "bar" {
    //     return Err(AuthError::WrongCredentials);
    // }
    let claims = Claims {
        // sub: "b@b.com".to_owned(),
        // company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    let jwt_keys = Keys::new(&state.config.jwt_secret.as_bytes());
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &jwt_keys.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        type Extractors = (Extension<Arc<State<'static>>>);

        let (Extension(state)) = Extractors::from_request(req)
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let jwt_keys = Keys::new(&state.config.jwt_secret.as_bytes());
        let token_data =
            decode::<Claims>(bearer.token(), &jwt_keys.decoding, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            _ => (StatusCode::BAD_REQUEST, "Unknown error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Claims {
    // pub(crate) sub: String,
    // pub(crate) company: String,
    pub(crate) exp: usize,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    account: String,
    code: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InvalidData,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidData => Some(&InvalidDataError {}),
            _ => None,
        }
    }
}

// impl From for AuthError {
//     fn from(error: std::io::Error) -> Self {
//         AuthError::InvalidData(io_error)
//     }
// }

#[derive(Debug)]
pub struct InvalidDataError;

impl Display for InvalidDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for InvalidDataError {}

#[derive(Debug)]
pub struct UnknownError;

impl Display for UnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for UnknownError {}
