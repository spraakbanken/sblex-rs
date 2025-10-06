use std::path::Path;

use fjall::{Config, Keyspace, PartitionCreateOptions, PartitionHandle, PersistMode};
use sblex_services::{models::lookup::LookupError, ports::Morphology, MorphologyBuilder};

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
        let value = if let Some(data) = self
            .saldo_morph
            .get(word)
            .map_err(|err| sblex_services::MorphologyBuilderError::Unknown(Box::new(err)))?
        {
            let mut new_value = data[..(data.len() - 1)].to_vec();
            new_value.push(b',');
            new_value.extend(value.as_bytes());
            new_value.push(b']');
            new_value
        } else {
            let mut new_value = b"[".to_vec();
            new_value.extend(value.as_bytes());
            new_value.push(b']');
            new_value
        };
        self.saldo_morph
            .insert(word, value)
            .map_err(|err| sblex_services::MorphologyBuilderError::Unknown(Box::new(err)))?;
        self.keyspace
            .persist(PersistMode::SyncAll)
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
    fn lookup(&self, fragment: &str) -> Result<Option<Vec<u8>>, LookupError> {
        Ok(self
            .saldo_morph
            .get(fragment)
            .map_err(|err| LookupError::Unknown(Box::new(err)))?
            .map(|bytes| bytes.to_vec()))
    }
    fn lookup_with_cont(&self, fragment: &str) -> Result<Vec<u8>, LookupError> {
        let mut conts: String = String::new();
        for kvpair in self.saldo_morph.prefix(fragment) {
            let (key, _value) = kvpair.unwrap();
            let key_str = std::str::from_utf8(&key).unwrap();
            if let Some(cont) = key_str.strip_prefix(fragment) {
                if let Some(c) = cont.chars().next() {
                    if !conts.contains(c) {
                        conts.push(c);
                    }
                }
            }
        }
        let mut result = b"{\"a\":".to_vec();
        if let Some(a) = self
            .saldo_morph
            .get(fragment)
            .map_err(|err| LookupError::Unknown(Box::new(err)))?
        {
            result.extend(a.iter());
        } else {
            result.extend(b"[]");
        }
        result.extend(b",\"c\":\"");
        result.extend(conts.as_bytes());
        result.extend(b"\"}");
        Ok(result)
    }
}
