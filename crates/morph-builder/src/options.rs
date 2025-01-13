use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to '.lex' file
    pub path: PathBuf,
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Clone, PartialEq, clap::Subcommand)]
pub enum Command {
    /// Use the FjallMorphology
    Fjall {
        /// Path to fjall database
        #[clap(long = "db")]
        db_path: PathBuf,
    },
    /// Use the original TrieMorphology
    Trie,
}
