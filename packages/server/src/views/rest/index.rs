use serde::Deserialize;
use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::{extract::Extension, http::StatusCode};
use serde_json::json;

use crate::handlers::State;
use crate::models::error::{AppError, HttpError, OtherError};
use crate::models::index::IndexModel;
use crate::{helpers, layers};

const INDEX_PAGE_SIZE: i32 = 10;

#[derive(Deserialize)]
pub struct IndexQuery {
    p: Option<i32>,
}

pub async fn index_handler<'a>(
    Query(args): Query<IndexQuery>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Html<String>, HttpError> {
    let mut current_page = args.p.unwrap_or(1);
    tracing::debug!("current_page:{}", current_page,);
    if current_page < 1 {
        return Err(HttpError::from(AppError::InvalidParameter));
    }

    let conn = state
        .pool
        .get()
        .await
        .map_err(|err| OtherError::Unknown(err))?;

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
where articles.status = 1
order by update_time desc offset $1 limit $2;",
            &[&offset, &limit],
        )
        .await
        .map_err(|err| OtherError::Unknown(err))?;

    let mut models: Vec<IndexModel> = Vec::new();

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

        let model = IndexModel {
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
        //println!("found article: {:?}", model);
        models.push(model);
    }
    let pages_html = helpers::calc_page_html(max_page, current_page);
    let result = state
        .registry
        .render(
            "index",
            &json!({ "models": models, "pages_html": pages_html }),
        )
        .map_err(|err| OtherError::Unknown(err))?;

    Ok(Html(result))
}
