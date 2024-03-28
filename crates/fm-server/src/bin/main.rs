use std::env;

use clap::Parser;
use eyre::Context;
use fm_server::{cli, config, server, startup, state::AppState, telemetry};

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
    telemetry::init_telemetry()?;
    // init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let args = cli::Options::parse();

    let state = AppState::from_path(&settings.morphology_path).with_context(|| format!("morphology_path: {}", &settings.morphology_path))?;

    let app = server::create_app(state);

    let address = format!("{}:{}", args.host, args.port);
    tracing::info!("Starting server at 'http://{}'", address);
    let listener = tokio::net::TcpListener::bind(address).await?;

    startup::run(listener, app).await?;

    Ok(())
}
