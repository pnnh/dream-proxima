use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::{extract::Extension, http::StatusCode};
use serde_json::json;

use crate::handlers::State;
use crate::models::error::{AppError, OtherError};
use crate::models::index::IndexModel;
use crate::service::index::IndexService;
use crate::views::restful::error::HttpRESTError;
use crate::{helpers, layers};

const INDEX_PAGE_SIZE: i32 = 10;

#[derive(Deserialize)]
pub struct IndexQuery {
    p: Option<i32>,
}

pub async fn index_handler<'a>(
    Query(args): Query<IndexQuery>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Html<String>, HttpRESTError> {
    let mut current_page = args.p.unwrap_or(1);
    tracing::debug!("current_page:{}", current_page,);
    if current_page < 1 {
        return Err(HttpRESTError::from(AppError::InvalidParameter));
    }

    let index_service = IndexService::new(state.clone());
    let count = index_service.query_count().await?;

    let row_count = count as i32;
    let mut max_page = row_count / INDEX_PAGE_SIZE;
    if row_count % INDEX_PAGE_SIZE != 0 {
        max_page += 1;
    }
    if current_page > max_page {
        current_page = max_page;
    }

    let offset: i64 = ((current_page - 1) * INDEX_PAGE_SIZE) as i64;
    let limit: i64 = INDEX_PAGE_SIZE as i64;

    let models = index_service.query(offset, limit).await?;

    let pages_html = helpers::calc_page_html(max_page, current_page);
    let result = state
        .registry
        .render(
            "index",
            &json!({ "models": models, "pages_html": pages_html }),
        )
        .map_err(|err| AppError::Handlebars(err))?;

    Ok(Html(result))
}
