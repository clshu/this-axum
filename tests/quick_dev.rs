#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jenny").await?.print().await?;
    // e.g. /hello2/Mike. Don't use /hello2/:name anymore
    hc.do_get("/hello2/Mike").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
