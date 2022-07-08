use crate::handlers::State;
use axum::response::Html;
use axum::{extract::Extension, http::StatusCode};
use serde_json::json;
use std::sync::Arc;

pub async fn about_handler(
    Extension(state): Extension<Arc<State<'_>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let result = state
        .registry
        .render("about", &json!({}))
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(Html(result))
}
