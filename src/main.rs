#![allow(unused)]

use axum::routing::get;
use axum::{Router, response::Html};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/hello",
        get(|| async { Html("Hello, <strong>World!</strong>") }),
    );

    // region:    --- Start Server

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    // endregion: --- Start Server
}
