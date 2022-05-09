extern crate libc;

use std::ffi::CStr;
use std::str;

use axum::response::Html;
use axum::routing::get;
use handlebars::Handlebars;
use libc::c_char;
use serde_json::json;

async fn index() -> Result<Html<String>, String> {
    let mut reg = Handlebars::new();
    let result = reg.render_template("Hello {{name}}", &json!({"name": "World"})).map_err(|err| err.to_string())?;
    println!(
        "{}",
        result
    );

    reg.register_template_string("tpl_1", "Good afternoon, {{name}}").map_err(|err| err.to_string())?;
    let result2 = reg.render("tpl_1", &json!({"name": "World"})).map_err(|err| err.to_string())?;
    println!("{}", result2);

    let html = result + "\n" + result2.as_str();

    Ok(Html(html))
}

async fn html_file() -> Result<Html<String>, String> {
    let mut reg = Handlebars::new();
    reg.register_template_file("index", "assets/templates/index.html").unwrap();
    let result = reg.render("index", &json!({"name": "World"}))
        .map_err(|err| err.to_string())?;
    println!("{}", result);

    Ok(Html(result))
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/html", get(index))
        .route("/file", get(html_file));

    axum::Server::bind(&"0.0.0.0:5500".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
