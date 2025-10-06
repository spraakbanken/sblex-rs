use std::path::Path;

use fjall_morphology::FjallMorphology;
use saldo_ws::http::{HttpServer, HttpServerConfig};
use sblex_services::{
    mem::MemLookupLid, morphology, ports::Morphology, service::Service, MorphologyBuilder,
};

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    fn load_or_build_morphology(db_path: &str, morph_path: &str) -> eyre::Result<FjallMorphology> {
        let mut morph = FjallMorphology::new(db_path)?;
        match morph.lookup("dväljes") {
            Ok(Some(_)) => Ok(morph),
            _ => {
                eprintln!("Morphology loading failed, building ...");

                morphology::build_from_path(&mut morph, morph_path)?;
                morph.finish()?;
                Ok(morph)
            }
        }
    }
    pub async fn spawn_app() -> eyre::Result<Self> {
        let saldo_morphology = Self::load_or_build_morphology(
            "data/testing/morphology.db",
            "assets/testing/saldo.lex",
        )?;
        let lookup_lid = MemLookupLid::from_tsv_path(Path::new("assets/testing/saldo.txt"))?;
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
