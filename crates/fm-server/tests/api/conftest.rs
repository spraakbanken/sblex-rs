use fm_server::http::{HttpServer, HttpServerConfig};
use reqwest::Url;
use trie_morphology::TrieMorphology;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn new() -> eyre::Result<TestApp> {
        let host = "127.0.0.1";

        let saldo_morphology = TrieMorphology::from_path("assets/testing/saldo.lex")?;
        let http_server_config = HttpServerConfig { port: 0, host };
        let http_server = HttpServer::new(saldo_morphology, http_server_config).await?;
        let port = http_server.local_addr()?.port();
        tokio::spawn(async move { http_server.run().await });
        let address = format!("http://{}:{}", host, port);

        Ok(Self { address })
    }

    pub fn url<S: AsRef<str>>(&self, path: S) -> eyre::Result<String> {
        let base = Url::parse(self.address.as_ref())?;
        let url = base.join(path.as_ref())?;
        Ok(url.as_str().to_string())
    }
}
