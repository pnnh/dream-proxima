use std::net::SocketAddr;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod graphql;
mod handlers;
mod helpers;
mod layers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 5500));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(handlers::app().await.into_make_service())
        .await
        .unwrap();
}
