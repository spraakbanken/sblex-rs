#[tokio::main]
async fn main() -> eyre::Result<()> {
    let hc = httpc_test::new_client("http://localhost:3003")?;

    hc.do_get("/health").await?.print().await?;

    hc.do_get("/version/json").await?.print().await?;

    hc.do_get("/lid/json/dväljas..1").await?.print().await?;
    hc.do_get("/lid/xml/dväljas..1").await?.print().await?;

    Ok(())
}
