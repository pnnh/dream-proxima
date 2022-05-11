use axum::{extract::Extension, http::StatusCode};

use crate::layers;

pub async fn using_connection_pool_extractor(
    Extension(pool): Extension<layers::ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(layers::internal_error)?;

    let query_result = conn
        .query("SELECT pk, title FROM articles limit 10", &[])
        .await
        .map_err(layers::internal_error)?;

    for row in query_result {
        let pk: &str = row.get(0);
        let title: &str = row.get(1);

        println!("found article: {} {}", pk, title);
    }

    Ok("ok".to_string())
}
