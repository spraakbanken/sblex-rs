use std::{
    env,
    path::{Path, PathBuf},
};

use eyre::Context;
use fjall_morphology::FjallMorphology;
use saldo_ws::http::{HttpServer, HttpServerConfig};
use sblex_services::{mem::MemLookupLid, service::Service};

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    fn get_db_path() -> PathBuf {
        project_root().join("data/testing/morphology.db")
    }
    fn get_lookup_path() -> PathBuf {
        project_root().join("assets/testing/saldo.txt")
    }
    pub async fn spawn_app() -> eyre::Result<Self> {
        let morph_path = Self::get_db_path();
        let saldo_morphology = FjallMorphology::new(&morph_path).wrap_err_with(|| {
            format!("Failed loading morphology from '{}'", morph_path.display())
        })?;
        let lookup_path = Self::get_lookup_path();
        let lookup_lid = MemLookupLid::from_tsv_path(&lookup_path).wrap_err_with(|| {
            format!("Failed loading LookupLid from '{}'", lookup_path.display())
        })?;
        let sblex_service = Service::new(lookup_lid, saldo_morphology);
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

    pub fn ff_json(&self, w: &str) -> String {
        format!("{}/ff/json/{}", self.address, w)
    }
}

/// Returns the path to the root directory of `sblex-rs` project.
fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}
