use std::{path::Path, sync::Arc};

use tokio::sync::RwLock;
use tracing::instrument;
use trie_morphology::fm::{self, Morphology};

#[derive(Debug, Clone)]
pub struct AppState {
    pub morphology: Arc<RwLock<Morphology>>,
}

impl AppState {
    #[instrument(skip(path))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<AppState, fm::Error> {
        Ok(Self {
            morphology: Arc::new(RwLock::new(Morphology::from_path(path)?)),
        })
    }
}
