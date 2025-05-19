use std::path::Path;

use saldo_ws::http::{HttpServer, HttpServerConfig};
use sblex_services::{mem::MemLookupLid, service::Service};

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn spawn_app() -> eyre::Result<Self> {
        let lookup_lid = MemLookupLid::from_tsv_path(&Path::new("assets/testing/saldo.txt"))?;
        let sblex_service = Service::new(lookup_lid);
        let host = "127.0.0.1";
        let http_server_config = HttpServerConfig { port: 0, host };
        let http_server = HttpServer::new(sblex_service, http_server_config).await?;

        let port = http_server.local_addr()?.port();
        let address = format!("http://{}:{}", host, port);

        tokio::spawn(async move { http_server.run().await });
        Ok(Self { address })
    }

    pub fn lid(&self, format: &str, w: &str) -> String {
        format!("{}/lid/{}/{}", self.address, format, w)
    }
}
