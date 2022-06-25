mod about;
mod account;
mod article;
mod index;
mod jwt;
mod sitemap;
mod user;

use axum::{extract::Extension, response::IntoResponse, routing::get, routing::post, Router};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use handlebars::Handlebars;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::http::Method;
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

use crate::{config, helpers, layers};

use crate::config::{is_debug, ProximaConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::Html;
use tower_http::cors::{Any, CorsLayer};

use crate::graphql::schema::{build_schema, AppSchema};
use crate::handlers::jwt::{authorize_handler, protected_handler};

#[derive(Clone, Debug)]
pub struct State<'reg> {
    pub registry: Handlebars<'reg>,
    pub pool: layers::ConnectionPool,
    pub config: ProximaConfig,
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn app() -> Router {
    let config = ProximaConfig::init().await.expect("初始化配置出错");

    let dsn_env: &str = config.dsn.as_str();

    let manager = PostgresConnectionManager::new_from_stringlike(dsn_env, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let mut reg = Handlebars::new();
    if is_debug() {
        reg.set_dev_mode(true);
    }
    reg.register_helper("reslink", Box::new(helpers::SimpleHelper));

    register_template_file(&mut reg);

    let state = Arc::new(State {
        registry: reg,
        pool,
        config,
    });

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let middleware = ServiceBuilder::new().add_extension(state.clone());

    let schema = build_schema(state.clone()).await;

    Router::new()
        .route("/", get(index::index_handler))
        .route("/about", get(about::about_handler))
        .route("/article/read/:pk", get(article::article_read_handler))
        .route(
            "/graphql",
            if config::is_debug() {
                get(graphql_playground).post(graphql_handler)
            } else {
                post(graphql_handler)
            },
        )
        .route("/user/:pk", get(user::user_info_handler))
        .route("/seo/sitemap", get(sitemap::sitemap_handler))
        .route("/protected", get(protected_handler))
        .route("/authorize", post(authorize_handler))
        .layer(cors)
        .layer(Extension(schema))
        .layer(middleware.into_inner())
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
