use std::env;

use clap::Parser;
use eyre::Context;
use fjall_morphology::FjallMorphology;
use fm_server::{
    cli, config,
    http::{HttpServer, HttpServerConfig},
    telemetry,
};
use sblex_services::{morphology, MorphologyBuilder};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::from_filename(".env");
    env::set_var(
        "OTEL_SERVICE_NAME",
        env::var("FM_SERVER__OTEL_SERVICE_NAME")?,
    );
    let settings = config::Settings::new()?;
    dbg!(&settings);
    // todo!("start");
    let _guard = telemetry::init_telemetry()?;
    // init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let args = cli::Options::parse();

    match args.cmd {
        cli::Command::Serve { port, host } => {
            let saldo_morphology = FjallMorphology::new(&settings.morphology_path)
                .with_context(|| format!("morphology_path: {}", &settings.morphology_path))?;

            let server_config = HttpServerConfig { port, host: &host };

            let http_server = HttpServer::new(saldo_morphology, server_config).await?;

            http_server.run().await?;
        }
        cli::Command::Db { path } => {
            let mut saldo_morphology = FjallMorphology::new(&settings.morphology_path)
                .with_context(|| format!("morphology_path: {}", &settings.morphology_path))?;
            morphology::build_from_path(&mut saldo_morphology, &path)?;
            saldo_morphology.finish()?;
        }
    }
    Ok(())
}
