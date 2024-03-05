use std::path::PathBuf;

use clap::Parser;
use fm_server::{server, startup, state::AppState};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;
    let args = Options::parse();

    let state = AppState::from_path(&args.saldo_morph_path)?;

    let app = server::create_app(state);

    let address = format!("{}:{}", args.host, args.port);
    eprintln!("Starting server at 'http://{}'", address);
    let listener = tokio::net::TcpListener::bind(address).await?;

    startup::run(listener, app).await?;

    Ok(())
}

#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about, long_about=None)]
struct Options {
    /// The host to bind the server to.
    #[arg(long, env = "FM_SERVER_APP_HOST", default_value = "127.0.0.1")]
    host: String,
    /// The port to bind the server to.
    #[arg(long, env = "FM_SERVER_APP_PORT", default_value = "3000")]
    port: u16,
    /// The path to the morphology data for saldo
    #[arg(
        long,
        env = "FM_SERVER_MORPH_SALDO_PATH",
        default_value = "assets/testing/saldo.lex"
    )]
    saldo_morph_path: PathBuf,
}
