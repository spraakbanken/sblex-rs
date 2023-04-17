use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3003")?;

    hc.do_get("/health").await?.print().await?;

    hc.do_get("/").await?.print().await?;
    Ok(())
}
