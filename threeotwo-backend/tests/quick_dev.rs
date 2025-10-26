
use anyhow::Result;
use axum::Json;
use serde_json::json;

#[tokio::test]
async fn test_fallback() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;
    client.do_get("/unrelated").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_wrong_login() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_post("/auth/login", json!({
        "user_name" : "admins",
        "password" : "password"
    })).await?.print().await?;
    
    client.do_get("/users/1").await?.print().await?;

    Ok(())

}

#[tokio::test]
async fn test_login() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_post("/auth/login", json!({
        "user_name" : "admin",
        "password" : "password"
    })).await?.print().await?;

    client.do_get("/users/1").await?.print().await?;

    Ok(())

}

#[tokio::test]
async fn test_user_create() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_post("/auth/login", json!({
        "user_name" : "admin",
        "password" : "password"
    })).await?.print().await?;

    client.do_post("/users", json!({
        "full_name": "JohnDimmermann"
    })).await?.print().await?;

    client.do_get("/users").await?.print().await?;
    Ok(())
   
}
