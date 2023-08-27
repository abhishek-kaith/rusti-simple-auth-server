#[tokio::test]
async fn dev()->anyhow::Result<()> {
    let client = httpc_test::new_client("http://0.0.0.0:8080").unwrap();
    client.do_get("/api/ping").await?.print().await?;
    Ok(())
}
