use crate::{Error, Result};
use axum::Router;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "admin" || payload.password != "1234" {
        return Err(Error::LoginFail);
    }

    // TODO: set cookie

    let body = json!({
       "result": {
         "success": true,
       }
    });

    Ok(Json(body))
}
