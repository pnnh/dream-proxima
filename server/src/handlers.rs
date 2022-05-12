use axum::response::Html;
use axum::{extract::Extension, http::StatusCode};
use handlebars::Handlebars;
use serde_json::json;

use crate::layers;
use crate::models;
use crate::models::ArticleView;

pub async fn using_connection_pool_extractor(
    Extension(pool): Extension<layers::ConnectionPool>,
) -> Result<Html<String>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(layers::internal_error)?;

    let query_result = conn
        .query("SELECT pk, title FROM articles limit 10", &[])
        .await
        .map_err(layers::internal_error)?;

    let mut models: Vec<models::ArticleView> = Vec::new();

    for row in query_result {
        let pk: &str = row.get(0);
        let title: &str = row.get(1);

        println!("found article: {} {}", pk, title);
        let model = ArticleView {
            pk: pk.to_string(),
            title: title.to_string(),
        };
        models.push(model);
    }

    let mut reg = Handlebars::new();
    reg.register_template_file("index", "assets/templates/index.hbs")
        .unwrap();
    reg.register_template_file("styles", "assets/templates/styles.hbs")
        .unwrap();

    Ok(Html(
        reg.render("index", &json!({ "models": models }))
            .map_err(layers::internal_error)?,
    ))
}
