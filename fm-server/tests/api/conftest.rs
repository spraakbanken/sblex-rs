use fm_server::{server, startup, state::AppState};
use reqwest::Url;
use tokio::net::TcpListener;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn new() -> eyre::Result<TestApp> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let state = AppState::default();
        let app = server::create_app(state);

        tokio::spawn(async move { startup::run(listener, app).await });
        Ok(Self { address })
    }

    pub fn url<S: AsRef<str>>(&self, path: S) -> eyre::Result<String> {
        let base = Url::parse(self.address.as_ref())?;
        let url = base.join(path.as_ref())?;
        Ok(url.as_str().to_string())
    }
}
