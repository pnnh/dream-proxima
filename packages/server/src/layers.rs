use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ProximaError {
    pub message: String,
}

impl ProximaError {
    pub fn new(message: &str) -> ProximaError {
        ProximaError {
            message: message.to_string(),
        }
    }
    pub fn from_string(message: String) -> ProximaError {
        ProximaError { message }
    }
}

impl Error for ProximaError {}

impl fmt::Display for ProximaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "proxima error: {}", self.message)
    }
}

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<ConnectionPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
