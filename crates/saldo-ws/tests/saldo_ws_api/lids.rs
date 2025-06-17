use crate::conftest::TestApp;
use reqwest::StatusCode;
use rstest::rstest;
use serde_json::Value as JsonValue;

#[rstest]
#[case("json")]
#[case("xml")]
#[case("html")]
#[tokio::test]
async fn test_invalid_input_returns_422(#[case] format: &str) -> eyre::Result<()> {
    // Arrange
    let app = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(app.lid(format, "bad-input"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let data: JsonValue = response.json().await?;
    // assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    insta::assert_json_snapshot!(format!("test_invalid_input_returns_422-{format}"), data);
    // assert_eq!(Some(15), response.content_length());
    Ok(())
}

#[rstest]
#[case("dväljas..1")]
#[case("dväljas..vb.1")]
#[tokio::test]
async fn test_json_valid_input_returns_200(#[case] lid: &str) -> eyre::Result<()> {
    // Arrange
    let app = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();
    // Act
    let response = client
        // Use the returned application address
        .get(app.lid("json", lid))
        .send()
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let data: JsonValue = response.json().await?;
    insta::assert_json_snapshot!(format!("test_json_valid_input_returns_200-{lid}"), data);
    Ok(())
}

#[rstest]
#[case("qt..1")]
#[case("qt..vb.1")]
#[tokio::test]
async fn test_json_missing_returns_404(#[case] lid: &str) -> eyre::Result<()> {
    // Arrange
    let app = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();
    // Act
    let response = client
        // Use the returned application address
        .get(app.lid("json", lid))
        .send()
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let data: JsonValue = response.json().await?;
    insta::assert_json_snapshot!(format!("test_json_missing_returns_404-{lid}"), data);
    Ok(())
}
