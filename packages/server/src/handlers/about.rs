use crate::handlers::State;
use crate::models::error::{DreamError, ProximaError};
use axum::response::Html;
use axum::{extract::Extension, http::StatusCode};
use serde_json::json;
use std::sync::Arc;

pub async fn about_handler(
    Extension(state): Extension<Arc<State>>,
) -> Result<Html<String>, ProximaError> {
    let result = state
        .registry
        .render("about", &json!({}))
        .map_err(|err| DreamError::Unknown(err))?;
    Ok(Html(result))
}
