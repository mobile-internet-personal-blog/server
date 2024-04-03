#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn qdev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8216")?;

    // hc.do_get("/").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "code": "6d30805d5225ba4b7b67",
            "third_party_provider": "Github"
        })
    );
    
    req_login.await?.print().await?;

    Ok(())
}