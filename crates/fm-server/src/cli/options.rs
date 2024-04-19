#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about, long_about=None)]
pub struct Options {
    /// The host to bind the server to.
    #[arg(long, env = "FM_SERVER_APP_HOST", default_value = "127.0.0.1")]
    pub host: String,
    /// The port to bind the server to.
    #[arg(long, env = "FM_SERVER_APP_PORT", default_value = "8765")]
    pub port: u16,
}
