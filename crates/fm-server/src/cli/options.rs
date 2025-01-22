use std::path::PathBuf;

#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about, long_about=None)]
pub struct Options {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    /// run the server
    Serve {
        /// The host to bind the server to.
        #[arg(long, env = "FM_SERVER_APP_HOST", default_value = "127.0.0.1")]
        host: String,
        /// The port to bind the server to.
        #[arg(long, env = "FM_SERVER_APP_PORT", default_value = "8765")]
        port: u16,
    },
    /// build the database from saldo.lex file
    Db {
        /// Path to '.lex' file
        path: PathBuf,
    },
}
