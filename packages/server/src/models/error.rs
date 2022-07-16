use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ProximaError {
    pub status: StatusCode,
    pub message: String,
}

impl ProximaError {
    pub fn new(message: &str) -> ProximaError {
        ProximaError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }
    pub fn from_string(message: String) -> ProximaError {
        ProximaError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl Display for ProximaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "proxima error: {}", self.message)
    }
}

impl<T> From<DreamError<T>> for ProximaError
// where
//     T: error::Error,
{
    fn from(error: DreamError<T>) -> Self {
        ProximaError::new(error.to_string().as_str())
    }
}

impl IntoResponse for ProximaError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
        }));
        (self.status, body).into_response()
    }
}

#[derive(Debug)]
pub struct RuntimeError;

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for RuntimeError {}

pub type SomeError = DreamError<RuntimeError>;
pub type OtherError<T> = DreamError<T>;

pub enum DreamError<T>
// where
//     T: error::Error,
{
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidData,
    InvalidToken,
    InvalidParameter,
    NotFound,
    EmptyData,
    Graphql(async_graphql::Error),
    BB8Postgres(bb8::RunError<T>),
    Postgresql(tokio_postgres::Error),
    Handlebars(handlebars::RenderError),
    Unknown(T),
}

impl<T> Display for DreamError<T>
// where
//     T: error::Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> Debug for DreamError<T>
// where
//     T: error::Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> std::error::Error for DreamError<T>
where
    T: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unknown(inner) => Some(inner),
            _ => None,
        }
    }
}

impl<T> From<T> for DreamError<T>
where
    T: error::Error,
{
    fn from(error: T) -> Self {
        Self::Unknown(error)
    }
}
