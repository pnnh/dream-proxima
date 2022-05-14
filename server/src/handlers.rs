use std::env;
use std::sync::Arc;

use axum::response::Html;
use axum::{
    extract::{Extension, Path},
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
use crate::{helpers, layers};

#[derive(Clone, Debug)]
struct State<'reg> {
    registry: Arc<Handlebars<'reg>>,
    pool: Arc<layers::ConnectionPool>,
}

fn register_template_file<'reg>(reg: &mut Handlebars) {
    reg.register_template_file("index", "assets/templates/pages/index.hbs")
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
        .route("/get", get(get_key))
        .layer(middleware.into_inner())
}

async fn get_key<'a>(
    Extension(state): Extension<State<'_>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(layers::internal_error)?;

    let offset: i32 = 0;
    let limit: i32 = 10;
    let query_result = conn
        .query(
            "select articles.pk, articles.title, articles.body, 
articles.description, articles.update_time, articles.creator, articles.keywords,
accounts.nickname, articles_views.views
from articles
    left join accounts on articles.creator = accounts.pk
	left join articles_views on articles.pk = articles_views.pk
order by update_time desc offset $1 limit $2;",
            &[&1i32, &10i32],
        )
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut models: Vec<IndexArticleView> = Vec::new();

    for row in query_result {
        let pk: &str = row.get(0);
        let title: &str = row.get(1);
        let body: &str = row.get(2);
        let description: &str = row.get(3);
        let update_time: chrono::NaiveDateTime = row.get(4);
        let creator: String = row.get(5);
        let keywords: String = row.get(6);
        let creator_nickname: &str = row.get(7);
        let views: i32 = row.get(8);

        let model = IndexArticleView {
            pk: pk.to_string(),
            title: title.to_string(),
            body: body.to_string(),
            description: description.to_string(),
            update_time,
            creator: creator.to_string(),
            creator_nickname: creator_nickname.to_string(),
            views,
            keywords,
        };
        println!("found article: {:?}", model);
        models.push(model);
    }

    let result = state
        .registry
        .render("index", &json!({ "models": models }))
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(result))
}
