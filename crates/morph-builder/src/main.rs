mod options;

use clap::Parser;
use fjall_morphology::FjallMorphology;
use miette::IntoDiagnostic;
use options::{Args, Command};
use sblex_services::{morphology, MorphologyBuilder};
use trie_morphology::trie::TrieBuilder;

fn main() -> miette::Result<()> {
    let args = Args::parse();
    dbg!(&args);
    let mut morph_builder: Box<dyn MorphologyBuilder> = match &args.cmd {
        Command::Trie => Box::new(TrieBuilder::default()),
        Command::Fjall { db_path } => Box::new(FjallMorphology::new(db_path).into_diagnostic()?),
    };
    morphology::build_from_path(morph_builder.as_mut(), &args.path)?;
    morph_builder.finish()?;

    Ok(())
}
