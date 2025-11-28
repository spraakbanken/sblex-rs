use crate::conftest::TestApp;
use reqwest::StatusCode;
use rstest::rstest;
use serde_json::Value as JsonValue;

#[rstest]
#[case("dväljs")]
#[tokio::test]
async fn test_json_valid_input_returns_200(#[case] segment: &str) -> eyre::Result<()> {
    // Arrange
    let app = TestApp::spawn_app().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client.get(app.fl_json(segment)).send().await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let data: JsonValue = response.json().await?;
    insta::assert_json_snapshot!(format!("test_json_valid_input_returns_200-{segment}"), data);
    Ok(())
}

//     #[rstest]
// #[case("dväljs")]
// #[case("dv")]
// #[case("dvä")]
// #[case("dväl")]
//     #[tokio::test]
//     async fn test_html_valid_input_returns_200(
//         #[case] fragment: &str
//     ) -> eyre::Result<()> {
//         res = await client.get(f"/ff/html?fragment={fragment}")
//         assert res.status_code == status.HTTP_200_OK
//         assert res.headers["content-type"] == "text/html; charset=utf-8"
//         assert res.text == snapshot
//             Ok(())
//     }
//
//     #[rstest]
// #[case("dväljs")]
// #[case("dv")]
// #[case("dvä")]
// #[case("dväl")]
//     #[tokio::test]
//     async fn test_html_orig_valid_input_returns_307(
//         #[case]fragment: &str,
//     ) -> eyre::Result<()> {
//         res = await client.get(f"/ff/html/{fragment}")
//         assert res.status_code == status.HTTP_307_TEMPORARY_REDIRECT
//             Ok(())
//     }
//
//     #[rstest]
// #[case("dväljs")]
// #[case("dv")]
// #[case("dvä")]
// #[case("dväl")]
//     #[tokio::test]
//     async fn test_xml_valid_input_returns_200(
//         #[case]fragment: &str
//     ) -> eyre::Result<()> {
//         res = await client.get(f"/ff/xml/{fragment}")
//         assert res.status_code == status.HTTP_200_OK
//         assert res.headers["content-type"] == "application/xml"
//         assert res.text == snapshot
//             Ok(())
//     }
