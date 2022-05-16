use std::env;
use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::{
    extract::Extension,
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    BoxError, Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

use crate::handlers::State;
use crate::{helpers, layers};
use std::collections::HashMap;

pub async fn article_read_handler<'a>(
    Path(params): Path<HashMap<String, String>>,
    Extension(state): Extension<State<'_>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let pk = params
        .get("pk")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "pk参数有误".to_string()))?;
    tracing::debug!("pk:{}", pk,);

    let conn = state.pool.get().await.map_err(layers::internal_error)?;

    let query_result = conn
        .query(
            "select articles.pk, articles.title, articles.body, 
articles.description, articles.update_time, articles.creator, articles.keywords,
accounts.nickname, articles_views.views
from articles
left join accounts on articles.creator = accounts.pk
left join articles_views on articles.pk = articles_views.pk
where articles.pk = $1;",
            &[&pk],
        )
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    if query_result.len() < 1 {
        return Err((StatusCode::NOT_FOUND, "文章未找到".to_string()));
    }

    let title: &str = query_result[0].get("title");
    let body: serde_json::Value = query_result[0].get("body");
    let description: &str = query_result[0].get("description");
    let update_time: chrono::NaiveDateTime = query_result[0].get("update_time");
    let creator: String = query_result[0].get("creator");
    let keywords: String = query_result[0].get("keywords");
    let creator_nickname: &str = query_result[0].get("nickname");
    let views: Option<i64> = query_result[0].get("views");

    let body_html = build_body(&body).or_else(|err| {
        tracing::error!("解析body出错: {}", err);
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "文章解析出错".to_string(),
        ))
    })?;

    let page_data = &json!({
        "pk": pk.to_string(),
        "title": title.to_string(),
        "body_html": body_html,
        "description": description.to_string(),
        "update_time_formatted": update_time.format("%Y年%m月%d日 %H:%M").to_string(),
        "creator": creator.to_string(),
        "creator_nickname": creator_nickname.to_string(),
        "views": views.unwrap_or(0),
        "keywords": keywords,
    });
    println!("page_data: {:?}", page_data);

    let result = state
        .registry
        .render("article_read", page_data)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(result))
}

fn build_body(nodes: &serde_json::Value) -> Result<String, String> {
    let children = nodes["children"]
        .as_array()
        .ok_or_else(|| "children未定义")?;

    let mut body_html_builder = string_builder::Builder::default();

    for child in children {
        let content = build_node(&child).or_else(|err| Err(err.to_string()))?;
        body_html_builder.append(content);
    }
    match body_html_builder.string() {
        Ok(v) => Ok(v),
        Err(err) => Err(err.to_string()),
    }
}

fn build_node(node: &serde_json::Value) -> Result<String, String> {
    let name = node["name"].as_str().ok_or_else(|| "未找到name属性")?;
    match name {
        "paragraph" => Ok(build_paragraph(node)?),
        "header" => Ok("header".to_string()),
        "code-block" => Ok("code-block".to_string()),
        _ => Err("undefined".to_string()),
    }
}

fn build_paragraph(node: &serde_json::Value) -> Result<String, String> {
    let children = node["children"]
        .as_array()
        .ok_or_else(|| "paragraph children未定义")?;

    let mut children_html_builder = string_builder::Builder::default();
    children_html_builder.append("<p class='fx-paragraph'>");
    for child in children {
        let content = build_text(&child).or_else(|err| Err(err.to_string()))?;
        children_html_builder.append(content.replace("\n", "<br/>"));
    }
    children_html_builder.append("</p>");
    match children_html_builder.string() {
        Ok(v) => Ok(v),
        Err(err) => Err(err.to_string()),
    }
}

fn build_text(node: &serde_json::Value) -> Result<String, String> {
    let text = node["text"].as_str().ok_or_else(|| "未找到text属性")?;

    let text_html = html_escape::encode_text(text);
    let mut text_decoration: String = "".to_string();
    let mut class_name: String = "".to_string();

    text_decoration.push_str(node["strike"].as_str().map_or("", |_| "line-through"));
    class_name.push_str(node["bold"].as_str().map_or("", |_| "fx-bold"));
    class_name.push_str(node["italic"].as_str().map_or("", |_| "fx-italic"));
    text_decoration.push_str(node["underline"].as_str().map_or("", |_| "underline"));
    class_name.push_str(node["code"].as_str().map_or("", |_| "fx-code"));

    let mut property: String = "".to_string();
    if !class_name.is_empty() {
        property = format!(" class='{}'", class_name);
    }
    if !text_decoration.is_empty() {
        property.push_str(format!(" style='text-decoration:{}'", text_decoration).as_str());
    }

    Ok(format!("<span {}>{}</span>", property, text_html))
}
