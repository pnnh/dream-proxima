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
        .route("/", get(index_handler))
        .layer(middleware.into_inner())
}

const INDEX_PAGE_SIZE: i32 = 10;

#[derive(Deserialize)]
struct IndexQuery {
    p: Option<i32>,
}

async fn index_handler<'a>(
    Query(args): Query<IndexQuery>,
    Extension(state): Extension<State<'_>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut current_page = args.p.unwrap_or(1);
    tracing::debug!("current_page:{}", current_page,);
    if current_page < 1 {
        return Err((StatusCode::BAD_REQUEST, "参数有误".to_string()));
    }

    let conn = state.pool.get().await.map_err(layers::internal_error)?;

    let row_count = 17;
    let mut max_page = row_count / INDEX_PAGE_SIZE;
    if row_count % INDEX_PAGE_SIZE != 0 {
        max_page += 1;
    }
    if current_page > max_page {
        current_page = max_page;
    }

    let offset: i64 = ((current_page - 1) * INDEX_PAGE_SIZE) as i64;
    let limit: i64 = INDEX_PAGE_SIZE as i64;

    let query_result = conn
        .query(
            "select articles.pk, articles.title, articles.body, 
articles.description, articles.update_time, articles.creator, articles.keywords,
accounts.nickname, articles_views.views
from articles
    left join accounts on articles.creator = accounts.pk
	left join articles_views on articles.pk = articles_views.pk
order by update_time desc offset $1 limit $2;",
            &[&offset, &limit],
        )
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut models: Vec<IndexArticleView> = Vec::new();

    for row in query_result {
        let pk: &str = row.get(0);
        let title: &str = row.get("title");
        let body: serde_json::Value = row.get(2);
        let description: &str = row.get("description");
        let update_time: chrono::NaiveDateTime = row.get(4);
        let creator: String = row.get(5);
        let keywords: String = row.get(6);
        let creator_nickname: &str = row.get(7);
        let views: Option<i64> = row.get(8);

        let model = IndexArticleView {
            pk: pk.to_string(),
            title: title.to_string(),
            body,
            description: description.to_string(),
            update_time_formatted: update_time.format("%Y年%m月%d日 %H:%M").to_string(),
            creator: creator.to_string(),
            creator_nickname: creator_nickname.to_string(),
            views: views.unwrap_or(0),
            keywords,
        };
        println!("found article: {:?}", model);
        models.push(model);
    }
    let pages_html = helpers::calc_page_html(max_page, current_page);
    let result = state
        .registry
        .render(
            "index",
            &json!({ "models": models, "pages_html": pages_html }),
        )
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(result))
}
