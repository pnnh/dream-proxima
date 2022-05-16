mod about;
mod account;
mod article;
mod index;
mod user;

use serde::Deserialize;
use std::env;
use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::{
    extract::Extension,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    BoxError, Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use handlebars::Handlebars;
use serde_json::json;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

use crate::models::IndexArticleView;
use crate::{handlers, helpers, layers};

#[derive(Clone, Debug)]
pub struct State<'reg> {
    registry: Arc<Handlebars<'reg>>,
    pool: Arc<layers::ConnectionPool>,
}

fn register_template_file<'reg>(reg: &mut Handlebars) {
    reg.register_template_file("index", "assets/templates/pages/index.hbs")
        .unwrap();
    reg.register_template_file("about", "assets/templates/pages/about.hbs")
        .unwrap();
    reg.register_template_file("styles", "assets/templates/partial/styles.hbs")
        .unwrap();
    reg.register_template_file("analytics", "assets/templates/partial/analytics.hbs")
        .unwrap();
    reg.register_template_file("footer", "assets/templates/partial/footer.hbs")
        .unwrap();
    reg.register_template_file("header", "assets/templates/partial/header.hbs")
        .unwrap();
    reg.register_template_file("headmeta", "assets/templates/partial/headmeta.hbs")
        .unwrap();
    reg.register_template_file("scripts", "assets/templates/partial/scripts.hbs")
        .unwrap();

    reg.register_template_file("article_read", "assets/templates/pages/article/read.hbs")
        .unwrap();
    reg.register_template_file("user_info", "assets/templates/pages/user/info.hbs")
        .unwrap();
}

pub async fn app() -> Router {
    let dsn_env = env::var("DSN").expect("dsn_env is error");

    let manager = PostgresConnectionManager::new_from_stringlike(dsn_env, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let mut reg = Handlebars::new();
    reg.register_helper("reslink", Box::new(helpers::SimpleHelper));

    register_template_file(&mut reg);

    let state = State {
        registry: Arc::new(reg),
        pool: Arc::new(pool),
    };

    let middleware = ServiceBuilder::new()
        // Share the state with each handler via a request extension
        .add_extension(state);

    // Build route service
    Router::new()
        .route("/", get(index::index_handler))
        .route("/about", get(about::about_handler))
        .route("/article/read/:pk", get(article::article_read_handler))
        .route("/user/:pk", get(user::user_info_handler))
        .layer(middleware.into_inner())
}
