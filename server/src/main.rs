extern crate libc;

use std::env;
use std::net::SocketAddr;

use axum::response::Html;
use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
    routing::get,
    Router,
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use handlebars::Handlebars;
use serde_json::json;
use tokio_postgres::NoTls;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn index() -> Result<Html<String>, String> {
    let mut reg = Handlebars::new();
    let result = reg.render_template("Hello {{name}}", &json!({"name": "World"})).map_err(|err| err.to_string())?;
    println!(
        "{}",
        result
    );

    reg.register_template_string("tpl_1", "Good afternoon, {{name}}").map_err(|err| err.to_string())?;
    let result2 = reg.render("tpl_1", &json!({"name": "World"})).map_err(|err| err.to_string())?;
    println!("{}", result2);

    let html = result + "\n" + result2.as_str();

    Ok(Html(html))
}

async fn html_file() -> Result<Html<String>, String> {
    let mut reg = Handlebars::new();
    reg.register_template_file("index", "assets/templates/index.html").unwrap();
    let result = reg.render("index", &json!({"name": "World"}))
        .map_err(|err| err.to_string())?;
    println!("{}", result);

    Ok(Html(result))
}

async fn postgres() -> Result<Html<String>, String> {
    use postgres::{Client, NoTls};
    let result = "hello".to_string();

    let dsn_env = env::var("DSN");
    // let dsn = match dsn_env {
    //     Ok(file) => file,
    //     Err(err) => return Ok(Html(err.to_string())),
    // };
    if dsn_env.is_err() {
        return Ok(Html(dsn_env.err().unwrap().to_string()));
    }
    let dsn = dsn_env.unwrap();

    let mut client = match tokio::task::spawn_blocking(move || {
        Client::connect(dsn.as_str(), NoTls)
    })
        .await
        .expect("client") {
        Ok(x) => x,
        Err(err) => return Ok(Html(err.to_string())),
    };


    let query_result = match tokio::task::spawn_blocking(move || {
        client.query("SELECT pk, title FROM articles limit 10", &[])
    })
        .await
        .expect("query_result") {
        Ok(x) => x,
        Err(error) => return Ok(Html(error.to_string())),
    };

    for row in query_result {
        let pk: &str = row.get(0);
        let title: &str = row.get(1);

        println!("found article: {} {}", pk, title);
    }
    Ok(Html(result))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();


    let dsn_env = env::var("DSN");
    if dsn_env.is_err() {
        tracing::error!("dsn_env is error {}", dsn_env.err().unwrap());
        return;
    }
    let dsn = dsn_env.unwrap();

    let manager =
        PostgresConnectionManager::new_from_stringlike(dsn, NoTls)
            .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();


    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/html", get(index))
        .route("/file", get(html_file))
        .route("/postgres", get(postgres))
        .route("/tokio_postgres", get(using_connection_pool_extractor).post(using_connection_extractor))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5500));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

// we can exact the connection pool with `Extension`
async fn using_connection_pool_extractor(
    Extension(pool): Extension<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

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

async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}