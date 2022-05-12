use std::env;
use std::net::SocketAddr;

use axum::response::Html;
use axum::{extract::Extension, routing::get};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use handlebars::Handlebars;
use serde_json::json;
use tokio_postgres::NoTls;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod layers;
mod models;

async fn index() -> Result<Html<String>, String> {
    let mut reg = Handlebars::new();
    reg.register_template_file("index", "assets/templates/index.hbs")
        .unwrap();
    let result = reg
        .render("index", &json!({"name": ["World", "啊啊啊"]}))
        .map_err(|err| err.to_string())?;
    println!("{}", result);

    Ok(Html(result))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let dsn_env = env::var("DSN");
    if dsn_env.is_err() {
        tracing::error!("dsn_env is error {}", dsn_env.err().unwrap());
        return;
    }
    let dsn = dsn_env.unwrap();

    let manager = PostgresConnectionManager::new_from_stringlike(dsn, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/hello", axum::routing::get(|| async { "Hello, World!" }))
        .route(
            "/tokio_postgres",
            get(handlers::using_connection_pool_extractor),
        )
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5500));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
