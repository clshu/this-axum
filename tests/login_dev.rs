#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn login_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "1234"
        }),
    );

    req_login.await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "12345"
        }),
    );

    req_login.await?.print().await?;

    Ok(())
}
