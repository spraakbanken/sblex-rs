use saldo_ws::http::{HttpServer, HttpServerConfig};
use sblex_services::{mem::MemLookupLid, service::Service};
use sblex_telemetry::telemetry;
use std::{env, path::Path};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::from_filename(".env");
    env::set_var(
        "OTEL_SERVICE_NAME",
        env::var("SALDO_WS__OTEL_SERVICE_NAME")?,
    );
    let server_config = HttpServerConfig {
        port: 3003,
        host: "127.0.0.1",
    };

    // let _guard = telemetry::init_telemetry()?;

    let lookup_lid = MemLookupLid::from_tsv_path(&Path::new("data/sblex/saldo.txt"))?;
    let sblex_service = Service::new(lookup_lid);

    let http_server = HttpServer::new(sblex_service, server_config).await?;
    // run it
    let address = http_server.local_addr()?;
    tracing::warn!("listening on {}", address);
    tracing::info!("try to call `curl -i {}` (with trace)", address); //Devskim: ignore DS137138
    tracing::info!("try to call `curl -i {}/health` (with NO trace)", address); //Devskim: ignore DS137138
    http_server.run().await?;
    Ok(())
}
