use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::format::format;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error;
use std::fmt::{Debug, Display, Formatter};

pub enum AppError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidData,
    InvalidToken,
    InvalidParameter,
    NotFound,
    EmptyData,
    InvalidConfig(&'static str),
    Graphql(async_graphql::Error),
    Postgresql(tokio_postgres::Error),
    Handlebars(handlebars::RenderError),
    Unknown(String),
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

impl<T> From<OtherError<T>> for AppError
where
    T: Debug,
{
    fn from(error: OtherError<T>) -> Self {
        AppError::Unknown(error.to_string())
    }
}

#[derive(Debug)]
pub enum OtherError<T>
where
    T: Debug,
{
    BB8Postgres(bb8::RunError<T>),
    Unknown(T),
}

impl<T> Display for OtherError<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
