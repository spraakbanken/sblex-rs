use std::path::Path;

use fjall::{Config, Keyspace, PartitionCreateOptions, PartitionHandle, PersistMode};
use sblex_services::{LookupError, Morphology, MorphologyBuilder};

#[derive(Clone)]
pub struct FjallMorphology {
    keyspace: Keyspace,
    saldo_morph: PartitionHandle,
}

impl FjallMorphology {
    pub fn new(folder: impl AsRef<Path>) -> Result<Self, fjall::Error> {
        let keyspace = Config::new(folder).open().unwrap();
        let saldo_morph =
            keyspace.open_partition("saldo_morph", PartitionCreateOptions::default())?;
        Ok(Self {
            keyspace,
            saldo_morph,
        })
    }
}

impl MorphologyBuilder for FjallMorphology {
    fn insert(
        &mut self,
        word: &str,
        value: String,
    ) -> Result<(), sblex_services::MorphologyBuilderError> {
        self.saldo_morph
            .insert(word, value)
            .map_err(|err| sblex_services::MorphologyBuilderError::Unknown(Box::new(err)))?;
        Ok(())
    }
    fn finish(&mut self) -> Result<(), sblex_services::MorphologyBuilderError> {
        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|err| sblex_services::MorphologyBuilderError::Unknown(Box::new(err)))?;
        Ok(())
    }
}
impl Morphology for FjallMorphology {
    fn lookup(&self, fragment: &str) -> Result<Option<Vec<u8>>, sblex_services::LookupError> {
        Ok(self
            .saldo_morph
            .get(fragment)
            .map_err(|err| LookupError::Unknown(Box::new(err)))?
            .map(|bytes| bytes.to_vec()))
    }
    fn lookup_with_state(&self, fragment: &str, state: usize) -> Option<&str> {
        todo!()
    }
}
