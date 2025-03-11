use crate::conftest::spawn_app;
use reqwest::StatusCode;
use rstest::rstest;

#[rstest]
#[case("json")]
#[case("xml")]
#[case("html")]
#[tokio::test]
async fn invalid_input_returns_400(#[case] format: &str) -> eyre::Result<()> {
    // Arrange
    let ctx = spawn_app().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/lid/{}/bad-input", &ctx.address, format))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    // assert_eq!(Some(15), response.content_length());
    Ok(())
}
