use crate::conftest::TestApp;
use serde_json::Value as JsonValue;

#[tokio::test]
async fn health_check_works() -> eyre::Result<()> {
    // Arrange
    let ctx = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health", &ctx.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let data: JsonValue = response.json().await?;
    insta::assert_json_snapshot!(data);
    Ok(())
}

#[tokio::test]
async fn version_works() -> eyre::Result<()> {
    // Arrange
    let ctx = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/version/json", &ctx.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let data = response
        .json::<JsonValue>()
        .await
        .expect("Failed to parse json");
    insta::assert_json_snapshot!(data);
    Ok(())
}
