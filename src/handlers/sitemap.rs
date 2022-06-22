use serde::Deserialize;
use std::env;
use std::io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write};
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
use chrono::{TimeZone, Utc};
use handlebars::Handlebars;
use serde_json::json;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

use crate::handlers::State;
use crate::{helpers, layers};

use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub async fn sitemap_handler<'a>(
    Extension(state): Extension<Arc<State<'_>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(layers::internal_error)?;

    let query_result = conn
        .query(
            "select articles.pk, articles.title, 
articles.description, articles.update_time
from articles
order by update_time desc;",
            &[],
        )
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let mut output = Cursor::new(Vec::new());
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut output);
    writer.write(
        XmlEvent::start_element("urlset")
            .attr("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"),
    );
    for row in query_result {
        let pk: &str = row.get("pk");
        let title: &str = row.get("title");
        let update_time: chrono::NaiveDateTime = row.get("update_time");
        let update_time_utc: chrono::DateTime<Utc> = Utc.from_utc_datetime(&update_time);
        let lastmod: String = update_time_utc.to_rfc3339();
        writer
            .write(XmlEvent::start_element("url"))
            .map_err(layers::internal_error)?;

        writer
            .write(XmlEvent::start_element("loc"))
            .map_err(layers::internal_error)?;
        writer
            .write(XmlEvent::characters(
                format!("https://sfx.xyz/article/read/{}", pk).as_str(),
            ))
            .map_err(layers::internal_error)?;
        writer
            .write(XmlEvent::end_element())
            .map_err(layers::internal_error)?;

        writer
            .write(XmlEvent::start_element("lastmod"))
            .map_err(layers::internal_error)?;
        writer
            .write(XmlEvent::characters(lastmod.as_str()))
            .map_err(layers::internal_error)?;
        writer
            .write(XmlEvent::end_element())
            .map_err(layers::internal_error)?;

        writer
            .write(XmlEvent::end_element())
            .map_err(layers::internal_error)?;
    }
    writer
        .write(XmlEvent::end_element())
        .map_err(layers::internal_error)?;
    output.seek(SeekFrom::Start(0)).unwrap();
    let mut result = String::new();
    output
        .read_to_string(&mut result)
        .map_err(layers::internal_error)?;

    Ok(Html(result))
}
