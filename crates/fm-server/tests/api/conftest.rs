use fm_server::http::{HttpServer, HttpServerConfig};
use reqwest::Url;
use sblex_services::{morphology, MorphologyBuilder};
use trie_morphology::{trie::TrieBuilder, TrieMorphology};

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn new() -> eyre::Result<TestApp> {
        let host = "127.0.0.1";

        let mut saldo_morph_builder = TrieBuilder::default();
        morphology::build_from_path(&mut saldo_morph_builder, "assets/testing/saldo.lex")?;
        saldo_morph_builder.finish()?;
        let saldo_morphology = TrieMorphology::new(saldo_morph_builder.build());

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
