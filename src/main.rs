#![allow(unused)]

use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::{Router, response::Html};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes_hello())
        .fallback_service(get_service(ServeDir::new("./")));
    // region:    --- Start Server

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    // endregion: --- Start Server
}

/*
In Axum 0.7+, you cannot use .nest_service("/", ...) because nesting at the root path is no longer supported.
Instead, you must use .fallback_service(...).
 */

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }

// region:    --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hadler_hello))
        // e.g. /hello2/Mike. Don't use /hello2/:name anymore
        .route("/hello2/{name}", get(hamdler_hello2))
}

// endregion: --- Routes Hello

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
