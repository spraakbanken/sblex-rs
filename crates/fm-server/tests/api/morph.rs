use reqwest::StatusCode;

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
    assert_eq!(status_code, StatusCode::OK);
    insta::assert_json_snapshot!(data);

    Ok(())
}

#[tokio::test]
async fn non_existent() -> eyre::Result<()> {
    // Arrange
    let app = TestApp::new().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client.get(app.url("/morph/löparsko/0")?).send().await?;

    // Assert
    let status_code = response.status();

    assert_eq!(status_code, StatusCode::NOT_FOUND);

    // let data: serde_json::Value = response.json().await?;
    // insta::assert_json_snapshot!(data);

    Ok(())
}
