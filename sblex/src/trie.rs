use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

pub struct Trie {
    trie: HashMap<usize, (HashMap<String, usize>, String)>,
}

impl Trie {
    pub fn builder() -> TrieBuilder {
        TrieBuilder::default()
    }

    pub fn lookup(&self, word: &str, start_state: usize) -> Option<&str> {
        let mut st = start_state;
        for c in word.graphemes(true) {
            st = match self.trie.get(&st) {
                Some(tuple) => match tuple.0.get(c) {
                    Some(state) => *state,
                    None => return None,
                },
                None => return None,
            };
        }
        self.trie.get(&st).map(|x| x.1.as_str())
    }
}

pub struct TrieBuilder {
    count: usize,
    trie: HashMap<usize, (HashMap<String, usize>, Vec<String>)>,
}

impl Default for TrieBuilder {
    fn default() -> Self {
        TrieBuilder {
            count: 0,
            trie: HashMap::new(),
        }
    }
}
impl TrieBuilder {
    pub fn build(self) -> Trie {
        let trie = HashMap::new();
        Trie { trie }
    }

    pub fn insert(&mut self, word: &str, decoration: String) {
        self.count += 1;
        let mut st = 0;
        for c in word.graphemes(true) {
            st = match self.trie.get(st) {
                Some(tuple) => match tuple.0.get(c) {
                    Some(state) => *state,
                },
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_trie() {
        let _trie = Trie::builder().build();
    }
}
