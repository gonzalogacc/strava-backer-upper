
use anyhow::Result;
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3007")?;
    hc.do_get("/login").await?.print().await?;

    Ok(())
}
