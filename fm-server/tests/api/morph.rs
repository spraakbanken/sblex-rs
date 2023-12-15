use reqwest::StatusCode;

use crate::conftest::TestApp;

#[tokio::test]
async fn can_call() -> eyre::Result<()> {
    // Arrange
    let app = TestApp::new().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client.get(app.url("/morph/örja/0")?).send().await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}
