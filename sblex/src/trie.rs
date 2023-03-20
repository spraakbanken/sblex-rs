use std::collections::HashMap;

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

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
    state: usize,
    trie: HashMap<usize, (HashMap<String, usize>, Vec<String>)>,
}

impl Default for TrieBuilder {
    fn default() -> Self {
        TrieBuilder {
            count: 0,
            state: 0,
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
        let mut iter = word.graphemes(true);
        loop {
            // for c in word.graphemes(true) {
            let c = match iter.next() {
                Some(c) => c,
                None => break,
            };
            st = match self.trie.get(&st) {
                Some(tuple) => match tuple.0.get(c) {
                    Some(state) => *state,
                    None => todo!(),
                },
                None => {
                    return self.complete(st, iter, decoration);
                }
            };
        }
        self.trie.get_mut(&st).unwrap().1.push(decoration);
    }

    // create a new branch
    fn complete(&mut self, st: usize, word: Graphemes, decoration: String) {
        for c in word {
            self.state += 1;
            self.trie.get_mut(&st).expect("st exists").0.get_mut(c)
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
