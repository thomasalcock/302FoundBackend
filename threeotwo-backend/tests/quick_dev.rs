
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_fallback() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;
    client.do_get("/unrelated").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_get("/user/create").await?.print().await?;

    client.do_get("/auth/login").await?.print().await?;

    client.do_post("/auth/login", json!({
        "user_name" : "admin",
        "password" : "password"
    })).await?.print().await?;
    
    client.do_post("/user/create", json!({})).await?.print().await?;

    Ok(())

}
