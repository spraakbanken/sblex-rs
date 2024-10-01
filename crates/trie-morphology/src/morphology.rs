use std::{fs, io};
use std::{io::BufRead, path::Path};

use sblex_services::Morphology;
use serde_json::{json, Value};
use tracing::instrument;

use crate::trie::Trie;
use crate::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrieMorphology {
    trie: Trie,
}

impl TrieMorphology {
    pub fn new(trie: Trie) -> Self {
        Self { trie }
    }

    #[instrument(skip(path))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut trie_builder = Trie::builder();
        for line in reader.lines() {
            let j: Value = serde_json::from_str(&line?)?;
            let w = &j["word"];
            let a = json!({
                "gf": j["head"],
                "id": j["id"],
                "pos": j["pos"],
                "is": j["inhs"],
                "msd": j["param"],
                "p": j["p"]
            });
            // w = j['word']
            // # a = '{"pos":"%s","is":[%s],"msd":"%s","p":"%s"}' % (j['pos'],"%s%s%s" % (cit(j['inhs']),'","'.join(j['inhs']),cit(j['inhs'])),j['param'],j['p'])
            // a = {
            //     "gf":j["head"],
            //     "id":j["id"],
            //     "pos":j["pos"],
            //     "is":j["inhs"],
            //     "msd":j["param"],
            //     "p":j["p"]
            // }
            // # % ("%s%s%s" % (
            // #         cit(j['inhs']),
            // #         '","'.join(j['inhs']),
            // #         cit(j['inhs'])),
            // #     j['param'],
            // #     j['p'])
            trie_builder.insert(w.as_str().expect("fm.morphology word"), a.to_string());
        }
        Ok(Self::new(trie_builder.build()))
    }

    pub fn lookup(&self, fragment: &str) -> Option<&str> {
        self.trie.lookup_with_state(fragment, 0)
    }
    pub fn lookup_with_state(&self, fragment: &str, state: usize) -> Option<&str> {
        self.trie.lookup_with_state(fragment, state)
    }
}

impl Morphology for TrieMorphology {
    fn lookup(&self, fragment: &str) -> Option<&str> {
        self.trie.lookup_with_state(fragment, 0)
    }

    fn lookup_with_state(&self, fragment: &str, state: usize) -> Option<&str> {
        self.trie.lookup_with_state(fragment, state)
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
