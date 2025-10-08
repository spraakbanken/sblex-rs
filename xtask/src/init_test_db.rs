use std::path::Path;
use std::path::PathBuf;

use fjall_morphology::FjallMorphology;
use sblex_services::morphology;
use sblex_services::MorphologyBuilder;

use crate::flags;

impl flags::InitTestDb {
    pub(crate) fn run(self, root_dir: &Path) -> eyre::Result<()> {
        let src_path: PathBuf = self
            .src
            .as_ref()
            .map(|p| p.into())
            .unwrap_or(root_dir.join("assets/testing/saldo.lex"));

        let dst_path = root_dir.join("data/testing/morphology.db");
        let mut morph = FjallMorphology::new(&dst_path)?;

        morphology::build_from_path(&mut morph, src_path)?;
        morph.finish()?;
        Ok(())
    }
}
