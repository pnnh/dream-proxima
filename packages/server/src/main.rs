use foo_rs::{testcall, testcall_cpp};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod graphql;
mod handlers;
mod helpers;
mod layers;
mod models;
mod service;
mod utils;
mod views;

#[tokio::main]
async fn main() {
    println!("Hello, world from Rust!");

    // calling the function from foo library
    unsafe {
        testcall(3.14159);
    };
    unsafe {
        testcall_cpp(3.14159);
    };
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(handlers::app().await.into_make_service())
        .await
        .unwrap();
}
