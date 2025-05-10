#![allow(unused)]

use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, response::Html};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hadler_hello))
        .route("/hello2/{name}", get(hamdler_hello2));

    // region:    --- Start Server

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    // endregion: --- Start Server
}

// region:    --- Handlers Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g. /hello?name=Jenny
async fn hadler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}!</strong>"))
}

// e.g. /hello2/Mike
async fn hamdler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    let name = name.as_str();
    Html(format!("Hello, <strong>{name}!</strong>"))
}

// endregion: --- Handlers Hello
