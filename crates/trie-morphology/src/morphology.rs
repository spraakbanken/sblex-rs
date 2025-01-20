use sblex_services::{LookupError, Morphology};

use crate::trie::Trie;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrieMorphology {
    trie: Trie,
}

impl TrieMorphology {
    pub fn new(trie: Trie) -> Self {
        Self { trie }
    }

    pub fn lookup(&self, fragment: &str) -> Option<&str> {
        self.trie.lookup_with_state(fragment, 0)
    }
    pub fn lookup_with_state(&self, fragment: &str, state: usize) -> Option<&str> {
        self.trie.lookup_with_state(fragment, state)
    }
}

impl Morphology for TrieMorphology {
    fn lookup(&self, fragment: &str) -> Result<Option<Vec<u8>>, sblex_services::LookupError> {
        if let Some(data) = self.trie.lookup_with_state(fragment, 0) {
            let value: serde_json::Value =
                serde_json::from_str(data).map_err(|err| LookupError::Unknown(Box::new(err)))?;
            if let Some(a) = value.get("a") {
                let a = a.as_array().unwrap();
                if a.is_empty() {
                    return Ok(None);
                }
                let data: Vec<u8> =
                    serde_json::to_vec(a).map_err(|err| LookupError::Unknown(Box::new(err)))?;
                Ok(Some(data))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    fn lookup_with_cont(&self, fragment: &str) -> Result<Vec<u8>, sblex_services::LookupError> {
        if let Some(data) = self.trie.lookup_with_state(fragment, 0) {
            Ok(data.into())
        } else {
            Err(LookupError::Unknown(
                format!("Found no data for '{}'", fragment).into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_morphology() {
        let _morph = TrieMorphology::new(Trie::builder().build());
    }
}
