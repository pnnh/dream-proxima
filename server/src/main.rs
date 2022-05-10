extern crate libc;

use std::env;

use axum::response::Html;
use axum::routing::get;
use handlebars::Handlebars;
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

async fn postgres() -> Result<Html<String>, String> {
    use postgres::{Client, NoTls};
    let result = "hello".to_string();

    let dsn_env = env::var("DSN");
    let dsn = match dsn_env {
        Ok(file) => file,
        Err(err) => return Ok(Html(err.to_string())),
    };

    println!("DSN is {}", dsn);

    let mut client = match tokio::task::spawn_blocking(move || {
        Client::connect(dsn.as_str(), NoTls)
    })
        .await
        .expect("client") {
        Ok(x) => x,
        Err(err) => return Ok(Html(err.to_string())),
    };


    let query_result = match tokio::task::spawn_blocking(move || {
        client.query("SELECT pk, title FROM articles limit 10", &[])
    })
        .await
        .expect("query_result") {
        Ok(x) => x,
        Err(error) => return Ok(Html(error.to_string())),
    };

    for row in query_result {
        let pk: &str = row.get(0);
        let title: &str = row.get(1);

        println!("found article: {} {}", pk, title);
    }
    Ok(Html(result))
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/html", get(index))
        .route("/file", get(html_file))
        .route("/postgres", get(postgres));

    axum::Server::bind(&"0.0.0.0:5500".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
