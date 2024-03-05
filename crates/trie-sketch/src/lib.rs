// pub mod naive_trie;
// pub mod trie;

// pub use trie::{Trie, TrieBuilder};

use std::collections::HashMap;

pub struct Trie {
    trie: HashMap<usize, (HashMap<char, usize>, String)>,
}

impl Trie {
    pub fn lookup(&self, word: &str) -> &str {
        self.lookup_with_state(word, 0)
    }
    pub fn lookup_with_state(&self, word: &str, start_state: usize) -> &str {
        let mut st = start_state;
        for c in word.chars() {
            st = match self.trie.get(&st) {
                None => return "",
                Some(state) => match state.0.get(&c) {
                    None => return "",
                    Some(state) => *state,
                },
            };
        }
        self.trie.get(&st).unwrap().1.as_str()
    }
}

#[derive(Debug)]
pub struct TrieBuilder {
    trie: HashMap<usize, (HashMap<char, usize>, Vec<String>)>,
    count: usize,
    state: usize,
}

impl Default for TrieBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl TrieBuilder {
    pub fn new() -> Self {
        let mut trie = HashMap::new();
        trie.insert(0, (HashMap::new(), vec![]));
        Self {
            trie,
            count: 0,
            state: 0,
        }
    }

    pub fn insert(&mut self, word: &str, decoration: &str) {
        self.count += 1;
        let mut st = 0;
        for (i, chr) in word.chars().enumerate() {
            dbg!(&chr);
            // for i in 0..word.len() {
            st = match self.trie.get(&st) {
                Some(state) => match state.0.get(&chr) {
                    Some(st) => *st,
                    None => {
                        self.complete(st, &word[i..], decoration);
                        return;
                    }
                },
                None => {
                    self.complete(st, &word[i..], decoration);
                    return;
                }
            }
        }
    }

    fn complete(&mut self, mut st: usize, word: &str, decoration: &str) {
        dbg!(word);
        for c in word.chars() {
            self.state += 1;
            self.trie.entry(st).and_modify(|e| {
                e.0.insert(c, self.state);
            });
            self.trie.insert(self.state, (HashMap::new(), Vec::new()));
            st = self.state;
        }
        self.trie.entry(st).and_modify(|e| {
            e.1.push(decoration.to_string());
        });
    }
    fn precompute(mut self) -> HashMap<usize, (HashMap<char, usize>, String)> {
        let mut trie_precomputed = HashMap::new();
        let mut max_num_transitions = 0;
        for i in 0..=self.state {
            let (tr, dec) = self.trie.remove(&i).unwrap();
            max_num_transitions = max_num_transitions.max(tr.len());
            let ys = dec.join(",");
            let cont = tr.keys().collect::<String>();
            dbg!(&cont);
            trie_precomputed.insert(i, (tr, format!(r#"{{"a":[{}],"c":"{}"}}"#, ys, cont)));
        }
        dbg!(&trie_precomputed);
        trie_precomputed
    }
    pub fn build(self) -> Trie {
        let trie = self.precompute();
        Trie { trie }
    }

    pub fn number_of_insertions(&self) -> usize {
        self.count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build() {
        let mut trie_builder = TrieBuilder::new();
        trie_builder.insert("ösja", r#"{"head":"ösja","pos":"vb"}"#);
        dbg!(&trie_builder);
        assert_eq!(trie_builder.number_of_insertions(), 1);
        let trie = trie_builder.build();

        assert_eq!(trie.lookup("ösj"), r#"{"a":[],"c":"a"}"#);
        assert_eq!(
            trie.lookup("ösja"),
            r#"{"a":[{"head":"ösja","pos":"vb"}],"c":""}"#
        );
    }
}
