use reqwest::StatusCode;
use serde_json::json;

use crate::conftest::TestApp;

#[tokio::test]
async fn can_call() -> eyre::Result<()> {
    // Arrange
    let app = TestApp::new().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client.get(app.url("/morph/dväljes/0")?).send().await?;

    // Assert
    let status_code = response.status();
    let data: serde_json::Value = response.json().await?;
    println!("{data:?}");
    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(
        data,
        json!({
            "a": [
                {"gf": "dväljas","id":"dväljas..vb.1","is":[],"msd":"pres ind s-form","p":"vb_vs_dväljas","pos":"vb"},
                {"gf": "dväljas","id":"dväljas..vb.1","is":[],"msd":"imper","p":"vb_vs_dväljas","pos":"vb"}
            ],
            "c": ""
        })
    );
    Ok(())
}
