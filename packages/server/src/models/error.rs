use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::format::format;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl HttpError {
    pub fn new(message: &str) -> HttpError {
        HttpError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }
    pub fn from_string(message: String) -> HttpError {
        HttpError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "proxima error: {}", self.message)
    }
}

impl<T> From<OtherError<T>> for HttpError {
    fn from(error: OtherError<T>) -> Self {
        HttpError::new(error.to_string().as_str())
    }
}

impl From<AppError> for HttpError {
    fn from(error: AppError) -> Self {
        match error {
            WrongCredentials => HttpError::new("授权有误"),
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
        }));
        (self.status, body).into_response()
    }
}

pub enum AppError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidData,
    InvalidToken,
    InvalidParameter,
    NotFound,
    EmptyData,
    Graphql(async_graphql::Error),
    Postgresql(tokio_postgres::Error),
    Handlebars(handlebars::RenderError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WrongCredentials => write!(f, "授权错误2"),
        }
    }
}

pub enum OtherError<T> {
    BB8Postgres(bb8::RunError<T>),
    Unknown(T),
}

impl<T> Display for OtherError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> Debug for OtherError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> std::error::Error for OtherError<T>
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

impl<T> From<T> for OtherError<T>
where
    T: error::Error,
{
    fn from(error: T) -> Self {
        Self::Unknown(error)
    }
}
