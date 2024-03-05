use reqwest::Url;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let test_app = TestApp::new("http://localhost:3000");

    get_and_print(&test_app, "docs/openapi.json").await?;
    get_and_print(&test_app, "/morph/lilja/0").await?;

    Ok(())
}

async fn get_and_print(test_app: &TestApp<'_>, path: &str) -> eyre::Result<()> {
    println!(">>> calling '{}' >>>", path);
    let data: serde_json::Value = test_app
        .client
        .get(test_app.url(path)?)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    println!(" {data:?}");
    println!("<<<");
    Ok(())
}

pub struct TestApp<'a> {
    pub client: reqwest::Client,
    base_url: &'a str,
}

impl<'a> TestApp<'a> {
    pub fn new(base_url: &'a str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub fn url<S: AsRef<str>>(&self, path: S) -> eyre::Result<String> {
        let base = Url::parse(self.base_url.as_ref())?;
        let url = base.join(path.as_ref())?;
        Ok(url.as_str().to_string())
    }
}
